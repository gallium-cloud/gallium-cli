mod format;

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
use crate::tasks_internal::qemu_img::ensure_qemu_img;
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
    let qemu_img = ensure_qemu_img().await.context(HelperCommandSnafu)?;

    let storage_api = api_client.storage_api();

    let export_filename = format!("{vol_name}_export{}", exp_format.as_ext());

    let ui = TransferProgressUi::init(format!("Exporting {vol_name} to {export_filename}"));

    let mtls_helper = MtlsCredentialHelper::new().context(HelperCommandSnafu)?;

    let path_params = ExportNbdVolumePathParams {
        cluster_id: source,
        kube_name: vol_name,
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
            target_file: PathBuf::from(export_filename),
            target_format: exp_format.as_qemu_img_fmt().to_string(),
        },
    };

    let progress_updater =
        CommandProgressUpdater::build_and_spawn(cmd_api, &submit_resp, "AWAIT_NBD_COMPLETION")?;

    let convert_task = qemu_img_convert(qemu_img.clone(), convert_cmd).await;
    transfer_progress_ui(convert_task, progress_updater, ui).await
}
