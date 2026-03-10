use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum HelperCommandError {
    #[snafu(transparent)]
    JsonError { source: serde_json::Error },
    #[snafu(transparent)]
    IoError { source: std::io::Error },
    #[snafu(transparent)]
    TaskPanicked { source: tokio::task::JoinError },
    #[snafu(display("Helper command returned invalid response: {reason}"))]
    InvalidResponse { reason: &'static str },
    #[snafu(display("qemu-img not found"))]
    QemuImgNotFound,
    #[snafu(whatever, display("{message}"))]
    UnhandledError {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}
