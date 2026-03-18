use crate::api::errors::ApiClientError;
use crate::helpers::helper_cmd_error::HelperCommandError;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum TaskError {
    #[snafu(display("Missing or invalid input for {field}"))]
    UserInputInvalid { field: &'static str },
    #[snafu(display("Value '{val}' for {field} invalid: {reason}"))]
    UserInputInvalidValueReason {
        val: String,
        field: &'static str,
        reason: &'static str,
    },
    #[snafu(display("Invalid state: {reason}"))]
    InvalidState { reason: &'static str },
    #[snafu(display("Required parameter missing: {reason}"))]
    RequiredParameterMissing { reason: &'static str },
    #[snafu(display("Invalid state for {command}: {reason}"))]
    InvalidStateForCommand {
        command: &'static str,
        reason: String,
    },
    #[snafu(display("Requested operation not supported ({op}): {reason}"))]
    RequestedOperationNotSupported { op: &'static str, reason: String },
    #[snafu(display("API Client Error"), context(false))]
    ApiClientError { source: ApiClientError },
    #[snafu(display("API Response missing expected field: {field}"))]
    ApiResponseMissingField { field: &'static str },
    #[snafu(display("Command {slug} failed: {cmd_type}"))]
    CommandFailure { slug: String, cmd_type: String },
    #[snafu(display("Command response for {cmd_type} missing or invalid"))]
    CommandResponseMissingOrInvalid {
        cmd_type: String,
        serde_err: Option<serde_json::Error>,
    },
    #[snafu(display("Helper command error"))]
    HelperCommand { source: HelperCommandError },
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
