use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum HelperCommandError {
    #[snafu(display("JSON Serialization/Deserialization error"), context(false))]
    JsonError { source: serde_json::Error },
    #[snafu(display("IO Error"), context(false))]
    IoError { source: std::io::Error },
    #[snafu(display("Task Panicked"), context(false))]
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
