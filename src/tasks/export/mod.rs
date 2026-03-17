mod format;
mod ui_confirm;
mod volume_scan;

use crate::api::ApiClient;
use crate::api::storage_api::entities::{ExportNbdVolumePathParams, VolumeNbdExportRequest};
use crate::args::GlobalArguments;
use crate::helpers::auth::get_login_response_for_saved_credentials;
use crate::helpers::cmd::cmd_progress::CommandProgressUpdater;
use crate::helpers::mtls::MtlsCredentialHelper;
use crate::helpers::nbd::poll_for_nbd_response;
use crate::task_common::error::HelperCommandSnafu;
use snafu::ResultExt;

use crate::helpers::qemu::{ConvertOperation, QemuImgConvert, qemu_img_convert};
use crate::helpers::ui::transfer_progress_ui::{TransferProgressUi, transfer_progress_ui};
use crate::task_common::error::TaskError;
use crate::tasks::export::format::ExportFormat;
use crate::tasks::export::ui_confirm::confirm_export;
use crate::tasks::export::volume_scan::{ScannedVolume, scan_volumes_for_export};
use crate::tasks_internal::qemu_img::ensure_qemu_img;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(clap::Args)]
pub struct ExportArguments {
    /// Deployment ID to export from
    #[arg(short, long)]
    pub source: String,

    /// Volume names / IDs to export
    #[arg(short = 'n', long)]
    pub vol: Vec<String>,

    /// Virtual machine names / Ids to export
    #[arg(short = 'v', long)]
    pub vm: Vec<String>,

    /// Format to export
    #[arg(short, long)]
    pub format: ExportFormat,

    #[arg(short, long)]
    /// Do not ask for confirmation - just go ahead if parameters are valid
    yes: bool,
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

    let volumes = scan_volumes_for_export(&api_client, &args).await?;
    if confirm_export(&volumes, &args)? {
        for volume in volumes {
            process(&api_client, args.source.clone(), volume, args.format).await?;
        }
    }

    Ok(())
}

async fn process(
    api_client: &Arc<ApiClient>,
    source: String,
    volume: ScannedVolume,
    exp_format: ExportFormat,
) -> Result<(), TaskError> {
    let qemu_img = ensure_qemu_img().await.context(HelperCommandSnafu)?;

    let storage_api = api_client.storage_api();

    let export_filename = format!("{}_export{}", volume.kube_name, exp_format.as_ext());

    let export_file: PathBuf;
    if let Some(dir_name) = volume.vm_name.as_deref() {
        tokio::fs::create_dir_all(&dir_name)
            .await
            .whatever_context::<_, TaskError>("create dir for vm exports")?;
        export_file = PathBuf::from(dir_name).join(&export_filename);
    } else {
        export_file = PathBuf::from(&export_filename);
    }

    let ui = TransferProgressUi::init(format!(
        "Exporting {} to {}",
        volume.kube_name,
        export_file.display()
    ));

    let mtls_helper = MtlsCredentialHelper::new().context(HelperCommandSnafu)?;

    let path_params = ExportNbdVolumePathParams {
        cluster_id: source,
        kube_name: volume.kube_name,
        kube_ns: "default".to_string(),
    };

    let req = VolumeNbdExportRequest {
        csr_base64: mtls_helper.get_csr_base64().context(HelperCommandSnafu)?,
    };

    let submit_resp = storage_api.export_nbd_volume(&path_params, &req).await?;

    let cmd_api = api_client.command_api();

    let mtls_helper = mtls_helper
        .poll_for_credentials(&cmd_api, &submit_resp)
        .await?;

    let nbd_tls_hostname = mtls_helper
        .read_server_cert_hostname()
        .context(HelperCommandSnafu)?;
    let cert_dir = mtls_helper
        .write_credentials()
        .await
        .context(HelperCommandSnafu)?
        .to_path_buf();

    ui.spinner_init.start("Waiting for deployment");

    let nbd = poll_for_nbd_response(&cmd_api, &submit_resp).await?;

    ui.spinner_init.stop("Deployment waiting for connection");

    let convert_cmd = QemuImgConvert {
        cert_dir,
        nbd_tls_hostname,
        nbd_host: nbd.host_ip,
        nbd_port: nbd.host_port,
        op: ConvertOperation::Export {
            target_file: export_file,
            target_format: exp_format.as_qemu_img_fmt().to_string(),
        },
    };

    let progress_updater =
        CommandProgressUpdater::build_and_spawn(cmd_api, &submit_resp, "AWAIT_NBD_COMPLETION")?;

    let convert_task = qemu_img_convert(qemu_img.clone(), convert_cmd).await;
    transfer_progress_ui(convert_task, progress_updater, ui).await
}
