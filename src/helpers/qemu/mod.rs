use crate::helpers::helper_cmd_error::HelperCommandError;
use qemu_img_cmd_types::info::QemuInfo;
use std::path::Path;

#[allow(unused)]
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
