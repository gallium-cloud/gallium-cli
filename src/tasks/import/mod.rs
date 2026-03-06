mod disk_pool;
mod param_helpers;
mod source_scan;

use crate::task_common::error::TaskError;
use crate::tasks::import::source_scan::{ImportSource, ScanResult, scan_import_sources};

use crate::api::ApiClient;
use crate::api::command_v2_api::entities::ApiCmdStatus;
use crate::api::storage_api::entities::{ImportNbdVolumePathParams, VolumeNbdImportRequest};
use crate::helpers::auth::get_login_response_for_saved_credentials;
use crate::helpers::cmd::cmd_progress::CommandProgressUpdater;
use crate::helpers::mtls::MtlsCredentialHelper;
use crate::helpers::nbd::poll_for_nbd_response;
use crate::helpers::qemu::{ConvertOperation, QemuImgConvert, qemu_img_convert};
use crate::tasks::import::disk_pool::{DiskPoolDetermination, determine_disk_pool};
use crate::tasks::import::param_helpers::{description, truncate_name};
use cliclack::{confirm, log, multi_progress, progress_bar, spinner};
use humansize::{BINARY, format_size};
use snafu::ResultExt;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(clap::Parser)]
pub(crate) struct ImportArguments {
    #[arg(short, long)]
    /// The ID or name of the disk pool where the import should be stored
    pool: Option<String>,

    #[arg(short, long, required = true)]
    /// Path to the image file or files to import
    source: Vec<PathBuf>,

    #[arg(short, long)]
    /// The deployment ID to import to
    target: String,

    #[arg(short, long)]
    /// Do not ask for confirmation - just go ahead if parameters are valid
    yes: bool,
}

pub(crate) async fn import_main(
    global_args: &crate::args::GlobalArguments,
    args: ImportArguments,
) -> Result<(), TaskError> {
    let api_client = global_args.build_api_client()?.with_access_token(
        get_login_response_for_saved_credentials(global_args)
            .await?
            .try_into()?,
    );

    let storage_api = api_client.storage_api();
    let disk_pool = determine_disk_pool(&storage_api, &args).await?;

    let mut import_sources = vec![];
    for source in args.source.iter() {
        let ScanResult { sources, warnings } = scan_import_sources(source).await?;
        for warning in warnings {
            log::warning(&warning).whatever_context::<_, TaskError>("writing to terminal")?;
        }
        import_sources.extend(sources);
    }

    if confirm_import(&import_sources, &disk_pool, &args)? {
        for source in import_sources {
            process(api_client.clone(), &args, &disk_pool, source).await?;
        }
    }

    Ok(())
}

fn confirm_import(
    sources: &[ImportSource],
    disk_pool: &DiskPoolDetermination,
    args: &ImportArguments,
) -> Result<bool, TaskError> {
    if sources.is_empty() {
        log::warning("Nothing to import.")
            .whatever_context::<_, TaskError>("writing to terminal")?;
        return Ok(false);
    }

    let summary = sources
        .iter()
        .map(|s| {
            format!(
                "  {} (format: {}, volume size: {})",
                s.name_part,
                s.reported_format,
                format_size(s.virtual_size_bytes, BINARY),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    log::info(format!(
        "The following {} will be imported to disk pool \"{}\":\n{}",
        if sources.len() == 1 { "file" } else { "files" },
        disk_pool
            .display_name
            .as_deref()
            .unwrap_or(&disk_pool.kube_name),
        summary,
    ))
    .whatever_context::<_, TaskError>("writing to terminal")?;

    if !args.yes {
        let proceed = confirm("Proceed with import?")
            .initial_value(true)
            .interact()
            .whatever_context::<_, TaskError>("reading confirmation")?;
        if !proceed {
            log::warning("Import cancelled.")
                .whatever_context::<_, TaskError>("writing to terminal")?;
            return Ok(false);
        }
    }

    Ok(true)
}

async fn process(
    api_client: Arc<ApiClient>,
    import_args: &ImportArguments,
    disk_pool: &DiskPoolDetermination,
    source: ImportSource,
) -> Result<(), TaskError> {
    let storage_api = api_client.storage_api();

    let multi = multi_progress(format!("Importing {}", source.name_part));
    let spinner_init = multi.add(spinner());
    let pb = multi.add(progress_bar(10000));
    let spinner_final = multi.add(spinner());

    spinner_init.start("Preparing import");

    let mtls_helper = MtlsCredentialHelper::new()?;

    let path_params = ImportNbdVolumePathParams {
        cluster_id: import_args.target.clone(),
        kube_ns: "default".into(),
    };

    let req = VolumeNbdImportRequest {
        csr_base64: mtls_helper.get_csr_base64()?,
        volume_description: Some(description(&source.name_part)),
        volume_size_gb: source.virtual_size_gb_round_up()?,
        volume_storage_class: disk_pool.kube_name.clone(),
        volume_name: Some(truncate_name(&source.name_part)),
        import_source_file_name: Some(source.name_part.clone()),
    };

    let submit_resp = storage_api.import_nbd_volume(&path_params, &req).await?;
    spinner_init.start("Waiting for volume");
    //TODO: Poll all the commands to provide more detailed status as import porgresses

    let cmd_api = api_client.command_api();

    let mtls_helper = mtls_helper
        .poll_for_credentials(&cmd_api, &submit_resp)
        .await?;

    let nbd_tls_hostname = mtls_helper.read_server_cert_hostname()?;
    let cert_dir = mtls_helper.write_credentials().await?.to_path_buf();

    spinner_init.start("Waiting for deployment");

    let nbd = poll_for_nbd_response(&cmd_api, &submit_resp).await?;

    spinner_init.stop("Deployment ready to receive import");

    let convert_cmd = QemuImgConvert {
        cert_dir,
        nbd_tls_hostname,
        nbd_host: nbd.host_ip,
        nbd_port: nbd.host_port,
        op: ConvertOperation::Import {
            source_file: source.file_path,
            source_format: source.reported_format,
        },
    };

    let progress_updater =
        CommandProgressUpdater::build_and_spawn(cmd_api, &submit_resp, "AWAIT_NBD_COMPLETION")?;

    let (progress, mut task) = qemu_img_convert(convert_cmd).await;

    let mut ui_tick = tokio::time::interval(tokio::time::Duration::from_millis(100));
    let mut backend_tick = tokio::time::interval(tokio::time::Duration::from_millis(5000));

    // qemu-img can take some time after progress has reached 100% before it will actually terminate.
    // (it is waiting for the other side to fsync out the file, among other things)
    // rather than show a progress bar stuck at 100%, switch to a spinner.
    let mut waiting_for_cmd_to_complete = false;

    loop {
        tokio::select! {
            _ = ui_tick.tick() => {
                if !waiting_for_cmd_to_complete {
                    let p = progress.read_progress();
                    pb.set_position(p as u64);
                    if p == 10000 {
                        pb.stop("Sending data");
                        waiting_for_cmd_to_complete = true;
                        spinner_final.start("Waiting for completion");
                    }
                }
            }
            _ = backend_tick.tick() => {
                progress_updater.update_progress(progress.read_progress()as f64, 10000.0);
            }
            r = &mut task => {
                return match QemuImgConvert::assert_ok(r) {
                    Ok(_) => {
                        progress_updater.complete(ApiCmdStatus::COMPLETE).await?;
                        spinner_final.stop("Import complete");
                        multi.stop();
                        Ok(())
                    }
                    Err(e) => {
                        progress_updater.complete(ApiCmdStatus::FAILED).await.ok();
                        spinner_final.error("Import failed");
                        multi.stop();

                        Err(e.into())
                    }
                };
            }
        }
    }
}
