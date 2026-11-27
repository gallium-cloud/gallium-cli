use crate::task_common::error::{FilesystemSnafu, TaskError};
use snafu::ResultExt;
use std::path::Path;

pub async fn read_document(path: &Path) -> Result<String, TaskError> {
    tokio::fs::read_to_string(path)
        .await
        .context(FilesystemSnafu { op: "read", path })
}
