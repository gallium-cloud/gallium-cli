use crate::api::errors::ApiClientError;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum TaskError {
    #[snafu(display("Missing or invalid input for {field}"))]
    UserInputInvalid { field: &'static str },
    #[snafu(transparent)]
    ApiClientError { source: ApiClientError },
    #[snafu(display("API Response missing expected field: {field}"))]
    ApiResponseMissingField { field: &'static str },
}
