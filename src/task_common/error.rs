use crate::api::errors::ApiClientError;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum TaskError {
    #[snafu(display("Missing or invalid input for {field}"))]
    UserInputInvalid { field: &'static str },
    #[snafu(display("Invalid state for {command}: {reason}"))]
    InvalidStateForCommand {
        command: &'static str,
        reason: String,
    },
    #[snafu(transparent)]
    ApiClientError { source: ApiClientError },
    #[snafu(display("API Response missing expected field: {field}"))]
    ApiResponseMissingField { field: &'static str },
    #[snafu(display("Failed to initialize {name}"))]
    Initialize {
        name: &'static str,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[snafu(whatever, display("{message}"))]
    UnhandledError {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}
