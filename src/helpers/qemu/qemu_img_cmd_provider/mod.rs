pub mod dl_win;

use crate::helpers::helper_cmd_error::HelperCommandError;
use std::path::PathBuf;
use which::which;

#[derive(Clone)]
pub struct QemuImgCmdProvider {
    pub bin_path: PathBuf,
}

impl QemuImgCmdProvider {
    pub async fn find_bin() -> Result<QemuImgCmdProvider, HelperCommandError> {
        if let Ok(bin_path) = std::env::var("QEMU_IMG_BIN").map(PathBuf::from) {
            return if tokio::fs::try_exists(&bin_path).await.unwrap_or(false) {
                Ok(QemuImgCmdProvider { bin_path })
            } else {
                Err(HelperCommandError::InvalidResponse {
                    reason: "QEMU_IMG_BIN env var is set but does not point to a file",
                })
            };
        }

        #[cfg(target_os = "windows")]
        return find_in_cache().await;

        #[cfg(not(target_os = "windows"))]
        return find_in_path().await;
    }
}

#[cfg(target_os = "windows")]
async fn find_in_cache() -> Result<QemuImgCmdProvider, HelperCommandError> {
    let bin_path = dl_win::cache_dir_qemu_img_exe_path()?;
    if tokio::fs::try_exists(&bin_path).await.unwrap_or(false) {
        Ok(QemuImgCmdProvider { bin_path })
    } else {
        Err(HelperCommandError::QemuImgNotFound)
    }
}
#[cfg(not(target_os = "windows"))]
async fn find_in_path() -> Result<QemuImgCmdProvider, HelperCommandError> {
    match tokio::task::spawn_blocking(|| which("qemu-img")).await? {
        Ok(bin_path) => Ok(QemuImgCmdProvider { bin_path }),
        Err(which::Error::CannotFindBinaryPath) => Err(HelperCommandError::QemuImgNotFound),
        Err(e) => Err(HelperCommandError::UnhandledError {
            message: format!("{e}"),
            source: Some(Box::new(e)),
        }),
    }
}
