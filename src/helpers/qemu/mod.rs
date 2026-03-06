mod convert_progress;

use crate::helpers::helper_cmd_error::HelperCommandError;
use crate::helpers::qemu::convert_progress::{QemuConvertProgressProvider, report_progress};
use duct::Expression;
use qemu_img_cmd_types::info::QemuInfo;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::sync::Arc;
use tokio::task::{JoinError, JoinHandle};

pub async fn qemu_img_info(path: &Path) -> Result<QemuInfo, HelperCommandError> {
    let path_as_os_str = path.as_os_str().to_os_string();
    let qemu_info_json = tokio::task::spawn_blocking(move || {
        duct::cmd!("qemu-img", "info", "--output", "json", &path_as_os_str)
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
    pub source_file: PathBuf,
    pub source_format: String,
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

    fn build_expression(&self) -> Expression {
        duct::cmd!(
            "qemu-img",
            "convert",
            "-p", //Display progress bar
            "-n", //Skip the creation of the target volume
            "-f",
            &self.source_format,
            &self.source_file,
            "--object",
            &self.tls_object_arg(),
            "--target-image-opts",
            &self.nbd_image_opts_arg(),
        )
    }
}

pub async fn qemu_img_convert(
    args: QemuImgConvert,
) -> (
    Arc<QemuConvertProgressProvider>,
    JoinHandle<Result<Option<Output>, std::io::Error>>,
) {
    let convert_progress_provider = Arc::new(QemuConvertProgressProvider::default());
    let convert_progress_provider2 = convert_progress_provider.clone();
    let task_handle = tokio::task::spawn_blocking(move || {
        let reader = args.build_expression().reader()?;
        report_progress(convert_progress_provider2, reader)
    });

    (convert_progress_provider, task_handle)
}
