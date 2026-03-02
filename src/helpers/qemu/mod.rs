use crate::helpers::helper_cmd_error::HelperCommandError;
use qemu_img_cmd_types::info::QemuInfo;
use std::path::{Path, PathBuf};

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
    pub async fn run(self) -> Result<(), HelperCommandError> {
        let target_image_opts = format!(
            "driver=nbd,host={},port={},tls-creds=tls0,tls-hostname={}",
            self.nbd_host, self.nbd_port, self.nbd_tls_hostname
        );

        let tls_object = format!(
            "tls-creds-x509,id=tls0,endpoint=client,dir={},priority={}",
            self.cert_dir.display(),
            TLS_PRIORITY
        );

        tokio::task::spawn_blocking(move || {
            duct::cmd!(
                "qemu-img",
                "convert",
                "-p", //Display progress bar
                "-n", //Skip the creation of the target volume
                "-f",
                &self.source_format,
                &self.source_file,
                "--object",
                tls_object,
                "--target-image-opts",
                target_image_opts
            )
            .run()
        })
        .await??;
        Ok(())
    }
}
