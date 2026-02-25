use crate::task_common::error::TaskError;
use snafu::ResultExt;
use std::path::PathBuf;

pub fn current_exe() -> Result<PathBuf, TaskError> {
    std::env::current_exe().whatever_context::<_, TaskError>("resolve binary path")
}
