use crate::task_common::error::TaskError;
use gallium_api_client::apis::Error as GeneratedError;
use reqwest::StatusCode;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ApiClientError {
    #[snafu(display("API error {status}: {body}"))]
    Api { status: StatusCode, body: String },

    #[snafu(display("Failed to decode response body"))]
    DecodeResponse { source: serde_json::Error },

    #[snafu(display("Failed to read response body"))]
    ReadResponseBody { source: std::io::Error },

    #[snafu(transparent)]
    Request { source: reqwest::Error },

    #[snafu(transparent)]
    Middleware { source: reqwest_middleware::Error },

    #[snafu(display("Invalid path segment: {val}"))]
    InvalidPathSegmentParameter { val: String },

    #[snafu(display(
        "Login session expired or revoked. Run `{} login` again.",
        clap::crate_name!()
    ))]
    SessionExpired,

    #[snafu(display("API response missing expected field: {field}"))]
    ResponseMissingField { field: &'static str },

    #[snafu(display("API Client Internal Error: {reason}"))]
    InternalError { reason: &'static str },
}

impl<T> From<GeneratedError<T>> for ApiClientError {
    fn from(error: GeneratedError<T>) -> Self {
        match error {
            GeneratedError::Reqwest(source) => Self::Request { source },
            GeneratedError::ReqwestMiddleware(reqwest_middleware::Error::Middleware(e)) => e
                .downcast::<ApiClientError>()
                .unwrap_or_else(|e| Self::Middleware {
                    source: reqwest_middleware::Error::Middleware(e),
                }),
            GeneratedError::ReqwestMiddleware(source) => Self::Middleware { source },
            GeneratedError::Serde(source) => Self::DecodeResponse { source },
            GeneratedError::Io(source) => Self::ReadResponseBody { source },
            GeneratedError::ResponseError(response) => Self::Api {
                status: response.status,
                body: truncate_body(&response.content),
            },
        }
    }
}

impl<T> From<GeneratedError<T>> for TaskError {
    fn from(error: GeneratedError<T>) -> Self {
        Self::ApiClientError {
            source: error.into(),
        }
    }
}

fn truncate_body(body: &str) -> String {
    const MAX: usize = 2048;
    if body.len() <= MAX {
        return body.to_string();
    }
    let truncated = unicode_ellipsis::truncate_str(body, MAX);
    format!("{truncated} ({} bytes total)", body.len())
}
