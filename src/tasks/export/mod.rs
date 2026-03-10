mod format;

use crate::api::ApiClient;
use crate::api::command_v2_api::entities::ApiCmdStatus;
use crate::api::storage_api::entities::{ExportNbdVolumePathParams, VolumeNbdExportRequest};
use crate::args::GlobalArguments;
use crate::helpers::auth::get_login_response_for_saved_credentials;
use crate::helpers::cmd::cmd_progress::CommandProgressUpdater;
use crate::helpers::mtls::MtlsCredentialHelper;
use crate::helpers::nbd::poll_for_nbd_response;

use crate::helpers::qemu::{ConvertOperation, QemuImgConvert, qemu_img_convert};
use crate::task_common::error::TaskError;
use crate::tasks::export::format::ExportFormat;
use crate::tasks_internal::qemu_img::ensure_qemu_img;
use cliclack::{multi_progress, progress_bar, spinner};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(clap::Args)]
pub struct ExportArguments {
    /// Deployment ID to export from
    #[arg(short, long)]
    pub source: String,

    /// Volume ID to export
    #[arg(short, long)]
    pub vol: String,

    /// Format to export
    #[arg(short, long)]
    pub format: ExportFormat,
}
pub async fn export_main(
    global_args: &GlobalArguments,
    args: ExportArguments,
) -> Result<(), TaskError> {
    let api_client = global_args.build_api_client()?.with_access_token(
        get_login_response_for_saved_credentials(global_args)
            .await?
            .try_into()?,
    );

    //TODO: confirm  parameters are correct, source volume exists,
    // free space for export, etc

    process(api_client, args.source, args.vol, args.format).await?;

    Ok(())
}

async fn process(
    api_client: Arc<ApiClient>,
    source: String,
    vol_name: String,
    exp_format: ExportFormat,
) -> Result<(), TaskError> {
    let qemu_img = ensure_qemu_img().await?;

    let storage_api = api_client.storage_api();

    let export_filename = format!("{vol_name}_export{}", exp_format.as_ext());

    let multi = multi_progress(format!("Exporting {vol_name} to {export_filename}"));
    let spinner_init = multi.add(spinner());
    let pb = multi.add(progress_bar(10000));
    let spinner_final = multi.add(spinner());

    let mtls_helper = MtlsCredentialHelper::new()?;

    let path_params = ExportNbdVolumePathParams {
        cluster_id: source,
        kube_name: vol_name,
        kube_ns: "default".to_string(),
    };

    let req = VolumeNbdExportRequest {
        csr_base64: mtls_helper.get_csr_base64()?,
    };

    let submit_resp = storage_api.export_nbd_volume(&path_params, &req).await?;

    let cmd_api = api_client.command_api();

    let mtls_helper = mtls_helper
        .poll_for_credentials(&cmd_api, &submit_resp)
        .await?;

    let nbd_tls_hostname = mtls_helper.read_server_cert_hostname()?;
    let cert_dir = mtls_helper.write_credentials().await?.to_path_buf();

    spinner_init.start("Waiting for deployment");

    let nbd = poll_for_nbd_response(&cmd_api, &submit_resp).await?;

    spinner_init.stop("Deployment waiting for connection");

    let convert_cmd = QemuImgConvert {
        cert_dir,
        nbd_tls_hostname,
        nbd_host: nbd.host_ip,
        nbd_port: nbd.host_port,
        op: ConvertOperation::Export {
            target_file: PathBuf::from(export_filename),
            target_format: exp_format.as_qemu_img_fmt().to_string(),
        },
    };

    let progress_updater =
        CommandProgressUpdater::build_and_spawn(cmd_api, &submit_resp, "AWAIT_NBD_COMPLETION")?;

    //TODO: this is copy-pasted from import, it should be factored out.
    // (but, does it need the same logic around waiting for completion?)
    let (progress, mut task) = qemu_img_convert(qemu_img.clone(), convert_cmd).await;

    let mut ui_tick = tokio::time::interval(tokio::time::Duration::from_millis(100));
    let mut backend_tick = tokio::time::interval(tokio::time::Duration::from_millis(5000));
    let mut waiting_for_completion = false;
    loop {
        tokio::select! {
            _ = ui_tick.tick() => {
                if !waiting_for_completion {
                    let p = progress.read_progress();
                    pb.set_position(p as u64);
                    if p == 10000 {
                        pb.stop("Sending data");
                        waiting_for_completion = true;
                        spinner_final.start("Waiting for completion");
                    }
                }
            }
            _ = backend_tick.tick() => {
                // TODO: inform backend of progress
            }
            r = &mut task => {
                return match QemuImgConvert::assert_ok(r) {
                    Ok(_) => {
                        progress_updater.complete(ApiCmdStatus::COMPLETE).await?;
                        spinner_final.stop("Export complete");
                        multi.stop();
                        Ok(())
                    }
                    Err(e) => {
                        progress_updater.complete(ApiCmdStatus::FAILED).await.ok();
                        spinner_final.error("Export failed");
                        multi.stop();

                        Err(e.into())
                    }
                };
            }
        }
    }
}
