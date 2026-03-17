use crate::helpers::helper_cmd_error::{HelperCommandError, IoErrorPerformingActionSnafu};
use cliclack::progress_bar;
use directories::BaseDirs;
use dlmgr::consumers::atomic_file_consumer_sha256::AtomicFileConsumerSha256;
use dlmgr::{DownloadTask, DownloadTaskBuilder};
use hex_literal::hex;
use snafu::ResultExt;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;
use url::Url;

const QEMU_IMG_WIN_URL: &str =
    "https://cl1.gallium-cdn.com/utils/qemu/win64/v10.2.1-20260305/qemu-img.exe";
const QEMU_IMG_WIN_SHA256: [u8; 32] =
    hex!("cfae8f5bbced4bc8ea2dc1ca9581349a23c30fc55221b5d8465b982b590e6330");

pub fn cache_dir_qemu_img_exe_path() -> Result<PathBuf, HelperCommandError> {
    let base_dirs = BaseDirs::new().ok_or_else(|| HelperCommandError::InvalidResponse {
        reason: "base directories not available",
    })?;
    let cache_dir = base_dirs.cache_dir();

    let cli_cache_dir = cache_dir.join("Gallium-CLI");

    let bin_dir = cli_cache_dir.join("bin");

    Ok(bin_dir.join("qemu-img.exe"))
}

async fn get_and_create_parent_dir(path: &Path) -> Result<(), HelperCommandError> {
    let parent_dir = path
        .parent()
        .ok_or_else(|| HelperCommandError::InvalidResponse {
            reason: "dir parent not available",
        })?;

    fs::create_dir_all(parent_dir)
        .await
        .context(IoErrorPerformingActionSnafu {
            action: "dir parent create",
        })?;

    Ok(())
}

pub async fn download_qemu_img() -> Result<(), HelperCommandError> {
    let task_builder = DownloadTaskBuilder::new();
    let qemu_img_exe_path = cache_dir_qemu_img_exe_path()?;

    get_and_create_parent_dir(&qemu_img_exe_path).await?;

    let (consumer, mut complete_notify) =
        AtomicFileConsumerSha256::new(qemu_img_exe_path, QEMU_IMG_WIN_SHA256)
            .await
            .whatever_context::<_, HelperCommandError>("setup download")?;

    let task: DownloadTask = task_builder
        .begin_download(
            Url::parse(QEMU_IMG_WIN_URL)
                .whatever_context::<_, HelperCommandError>("parse download url")?
                .into(),
            consumer,
        )
        .await
        .whatever_context::<_, HelperCommandError>("begin download")?;

    let p = task.progress_provider();

    let mut ui_tick = tokio::time::interval(tokio::time::Duration::from_millis(100));

    let progress = progress_bar(100);

    progress.start("Downloading qemu-img");

    loop {
        tokio::select! {
            _ = ui_tick.tick() => {
                progress.set_position(p.progress_percent() as u64);
            }
            r = &mut complete_notify => {
                progress.set_position(100);
                r.whatever_context::<_, HelperCommandError>("await completion message")?
                .whatever_context::<_, HelperCommandError>("validate qemu-img")?;
                break;
            }
        }
    }

    progress.stop("Download complete");

    Ok(())
}
