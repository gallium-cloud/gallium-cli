use crate::helpers::qemu::qemu_img_cmd_provider::QemuImgCmdProvider;
use crate::helpers::qemu::qemu_img_info;
use crate::task_common::error::{HelperCommandSnafu, TaskError};
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
pub struct ScanResult {
    pub sources: Vec<ImportSource>,
    pub warnings: Vec<String>,
}

pub async fn scan_import_sources(
    qemu_img: &QemuImgCmdProvider,
    source: &Path,
) -> Result<ScanResult, TaskError> {
    let source_metadata = fs::metadata(source)
        .await
        .whatever_context::<_, TaskError>("Query filesystem metadata")?;

    if source_metadata.is_dir() {
        scan_directory(qemu_img, source).await
    } else if source_metadata.is_file() {
        Ok(ScanResult {
            sources: vec![scan_file(qemu_img, source).await?],
            warnings: vec![],
        })
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

const SUPPORTED_EXTENSIONS: &[&str] = &["qcow2", "vmdk", "img", "vhd", "vhdx"];

async fn scan_directory(
    qemu_img: &QemuImgCmdProvider,
    dir: &Path,
) -> Result<ScanResult, TaskError> {
    let mut entries = fs::read_dir(dir)
        .await
        .whatever_context::<_, TaskError>("Read source directory")?;

    let mut sources = vec![];
    let mut warnings = vec![];
    while let Some(entry) = entries
        .next_entry()
        .await
        .whatever_context::<_, TaskError>("Read directory entry")?
    {
        let path = entry.path();
        let is_supported = path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| SUPPORTED_EXTENSIONS.contains(&ext));
        if !is_supported {
            continue;
        }
        match scan_file(qemu_img, &path).await {
            Ok(source) => sources.push(source),
            Err(e) => {
                warnings.push(format!(
                    "Skipping {:?}: {}",
                    path.file_name().unwrap_or_default(),
                    e
                ));
            }
        }
    }

    Ok(ScanResult { sources, warnings })
}

async fn scan_file(
    qemu_img: &QemuImgCmdProvider,
    file_path: &Path,
) -> Result<ImportSource, TaskError> {
    let info = qemu_img_info(qemu_img.clone(), file_path)
        .await
        .context(HelperCommandSnafu)?;
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
