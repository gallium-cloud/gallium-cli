use crate::helpers::qemu::qemu_img_info;
use crate::task_common::error::TaskError;
use snafu::ResultExt;
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Clone, Debug)]
pub struct ImportSource {
    pub file_path: PathBuf,
    pub name_part: String,
    pub reported_format: String,
    pub virtual_size_bytes: u64,
}
pub async fn scan_import_sources(source: &Path) -> Result<Vec<ImportSource>, TaskError> {
    let source_metadata = fs::metadata(source)
        .await
        .whatever_context::<_, TaskError>("Query filesystem metadata")?;

    if source_metadata.is_dir() {
        todo!("scan directory");
    } else if source_metadata.is_file() {
        Ok(vec![scan_file(source).await?])
    } else if source_metadata.is_symlink() {
        Err(TaskError::RequestedOperationNotSupported {
            op: "import",
            reason: "Source file is a symlink".to_string(),
        })
    } else {
        //TODO importing a device node should be supported
        Err(TaskError::RequestedOperationNotSupported {
            op: "import",
            reason: "Source file type not supported".to_string(),
        })
    }
}

async fn scan_file(file_path: &Path) -> Result<ImportSource, TaskError> {
    let info = qemu_img_info(file_path).await?;
    let name_part = file_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .ok_or_else(|| TaskError::UnhandledError {
            message: "Couldn't resolve file name".to_string(),
            source: None,
        })?;
    Ok(ImportSource {
        file_path: file_path.to_path_buf(),
        name_part,
        reported_format: info.format,
        virtual_size_bytes: info.virtual_size,
    })
}

const ONE_GB: u64 = 1024 * 1024 * 1024;
impl ImportSource {
    pub fn virtual_size_gb_round_up(&self) -> Result<u32, TaskError> {
        self.virtual_size_bytes
            .div_ceil(ONE_GB)
            .try_into()
            .map_err(|_| TaskError::InvalidState {
                reason: "virtual_size_bytes integer overflow",
            })
    }
}
