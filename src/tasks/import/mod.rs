mod disk_pool;
mod source_scan;
use crate::task_common::error::TaskError;
use crate::tasks::import::source_scan::{ImportSource, scan_import_sources};

use crate::api::ApiClient;
use crate::api::storage_api::entities::{ImportNbdVolumePathParams, VolumeNbdImportRequest};
use crate::helpers::auth::get_login_response_for_saved_credentials;
use crate::helpers::mtls::MtlsCredentialHelper;
use crate::helpers::nbd::poll_for_nbd_response;
use crate::helpers::qemu::QemuImgConvert;
use crate::tasks::import::disk_pool::{DiskPoolDetermination, determine_disk_pool};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(clap::Parser)]
pub(crate) struct ImportArguments {
    #[arg(short, long)]
    /// The ID or name of the disk pool where the import should be stored
    pool: Option<String>,

    #[arg(short, long)]
    /// Path to the image file or files to import
    source: PathBuf,

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

    let sources = scan_import_sources(&args.source).await?;

    for source in sources {
        process(api_client.clone(), &args, &disk_pool, source).await?;
    }

    Ok(())
}

async fn process(
    api_client: Arc<ApiClient>,
    import_args: &ImportArguments,
    disk_pool: &DiskPoolDetermination,
    source: ImportSource,
) -> Result<(), TaskError> {
    let storage_api = api_client.storage_api();

    let mtls_helper = MtlsCredentialHelper::new()?;

    let path_params = ImportNbdVolumePathParams {
        cluster_id: import_args.target.clone(),
        kube_ns: "default".into(),
    };

    let req = VolumeNbdImportRequest {
        csr_base64: mtls_helper.get_csr_base64()?,
        volume_description: None,
        volume_size_gb: source.virtual_size_gb_round_up()?,
        volume_storage_class: disk_pool.kube_name.clone(),
        volume_name: Some(source.name_part.clone()),
        import_source_file_name: Some(source.name_part.clone()),
    };

    let submit_resp = storage_api.import_nbd_volume(&path_params, &req).await?;

    let cmd_api = api_client.command_api();

    let mtls_helper = mtls_helper
        .poll_for_credentials(&cmd_api, &submit_resp)
        .await?;

    let nbd_tls_hostname = mtls_helper.read_server_cert_hostname()?;
    let cert_dir = mtls_helper.write_credentials().await?.to_path_buf();

    let nbd = poll_for_nbd_response(&cmd_api, &submit_resp).await?;

    let convert_cmd = QemuImgConvert {
        cert_dir,
        nbd_tls_hostname,
        nbd_host: nbd.host_ip,
        nbd_port: nbd.host_port,
        source_file: source.file_path,
        source_format: source.reported_format,
    };

    convert_cmd.run().await?;

    eprintln!("Import completed successfully");

    Ok(())
}
