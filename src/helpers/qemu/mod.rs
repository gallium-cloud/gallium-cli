mod convert_progress;
pub mod qemu_img_cmd_provider;

use crate::helpers::helper_cmd_error::HelperCommandError;
use crate::helpers::qemu::convert_progress::{QemuConvertProgressProvider, report_progress};
use crate::helpers::qemu::qemu_img_cmd_provider::QemuImgCmdProvider;
use duct::Expression;
use qemu_img_cmd_types::info::QemuInfo;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::sync::Arc;
use tokio::task::{JoinError, JoinHandle};

pub async fn qemu_img_info(
    qemu_img: QemuImgCmdProvider,
    path: &Path,
) -> Result<QemuInfo, HelperCommandError> {
    let path_as_os_str = path.as_os_str().to_os_string();
    let qemu_info_json = tokio::task::spawn_blocking(move || {
        duct::cmd!(
            qemu_img.bin_path,
            "info",
            "--output",
            "json",
            &path_as_os_str
        )
        .stdout_capture()
        .read()
    })
    .await??;

    Ok(serde_json::from_str(&qemu_info_json)?)
}

const TLS_PRIORITY: &str = "SECURE128:-VERS-ALL:+VERS-TLS1.3";

pub struct QemuImgConvert {
    pub cert_dir: PathBuf,
    pub nbd_tls_hostname: String,
    pub nbd_host: String,
    pub nbd_port: u16,
    pub op: ConvertOperation,
}

// Eventually I'd like to build a proper abstraction here - the QemuImgConvert should take a
// Source and Target - but that will lead to a lot more scenarios that will need to be tested.
// For now we'll just have separate operations for Import (Local -> NBD) and Export (NBD -> Local)
// to keep things simpler.
pub enum ConvertOperation {
    Import {
        source_file: PathBuf,
        source_format: String,
    },
    Export {
        target_file: PathBuf,
        target_format: String,
    },
}

impl QemuImgConvert {
    pub fn assert_ok(
        r: Result<Result<Option<Output>, std::io::Error>, JoinError>,
    ) -> Result<(), HelperCommandError> {
        r??;

        Ok(())
    }
    fn tls_object_arg(&self) -> String {
        format!(
            "tls-creds-x509,id=tls0,endpoint=client,dir={},priority={}",
            self.cert_dir.display(),
            TLS_PRIORITY
        )
    }

    /// Format the nbd settings into the required format for either --image-opts or --target-image-opts
    fn nbd_image_opts_arg(&self) -> String {
        format!(
            "driver=nbd,host={},port={},tls-creds=tls0,tls-hostname={}",
            self.nbd_host, self.nbd_port, self.nbd_tls_hostname
        )
    }

    fn build_expression(&self, qemu_img: QemuImgCmdProvider) -> Expression {
        match self.op {
            ConvertOperation::Import {
                ref source_file,
                ref source_format,
            } => {
                duct::cmd!(
                    &qemu_img.bin_path,
                    "convert",
                    "-p", //Display progress bar
                    "-n", //Skip the creation of the target volume
                    "-f",
                    source_format,
                    source_file,
                    "--object",
                    &self.tls_object_arg(),
                    "--target-image-opts",
                    &self.nbd_image_opts_arg(),
                )
            }
            ConvertOperation::Export {
                ref target_file,
                ref target_format,
            } => {
                duct::cmd!(
                    &qemu_img.bin_path,
                    "convert",
                    "-p", //Display progress bar
                    "--object",
                    &self.tls_object_arg(),
                    "--image-opts",
                    &self.nbd_image_opts_arg(),
                    "-O",
                    target_format,
                    target_file,
                )
            }
        }
    }
}

pub struct ConvertTask {
    pub progress: Arc<QemuConvertProgressProvider>,
    pub handle: JoinHandle<Result<Option<Output>, std::io::Error>>,
}
pub async fn qemu_img_convert(qemu_img: QemuImgCmdProvider, args: QemuImgConvert) -> ConvertTask {
    let convert_progress_provider = Arc::new(QemuConvertProgressProvider::default());
    let convert_progress_provider2 = convert_progress_provider.clone();
    let task_handle = tokio::task::spawn_blocking(move || {
        let reader = args.build_expression(qemu_img).reader()?;
        report_progress(convert_progress_provider2, reader)
    });

    ConvertTask {
        progress: convert_progress_provider,
        handle: task_handle,
    }
}
