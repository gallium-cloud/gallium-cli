use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum HelperCommandError {
    #[snafu(transparent)]
    JsonError { source: serde_json::Error },
    #[snafu(transparent)]
    IoError { source: std::io::Error },
    #[snafu(transparent)]
    TaskPanicked { source: tokio::task::JoinError },
}
