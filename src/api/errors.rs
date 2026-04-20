use crate::api::common_api::entities::GalliumApiErrorResponse;
use reqwest::StatusCode;
use snafu::prelude::*;
use url::Url;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ApiClientError {
    #[snafu(display("API error {status} from {url}: {error:?}"))]
    Api {
        status: StatusCode,
        url: Url,
        error: GalliumApiErrorResponse,
    },
    #[snafu(display("Failed to decode response body ({status}) from {url}. Body: {body}"))]
    DecodeResponse {
        status: StatusCode,
        url: Url,
        body: String,
        source: serde_json::Error,
    },
    #[snafu(display("Failed to decode error response ({status}) from {url}. Body: {body}"))]
    DecodeErrorResponse {
        status: StatusCode,
        url: Url,
        body: String,
        source: serde_json::Error,
    },
    #[snafu(display("Failed to read response body ({status}) from {url}"))]
    ReadResponseBody {
        status: StatusCode,
        url: Url,
        source: reqwest::Error,
    },
    #[snafu(transparent)]
    Request { source: reqwest::Error },
    #[snafu(transparent)]
    UrlParseError { source: url::ParseError },
    // Indicates a bug in the CLI tool.
    #[snafu(display("API Client Internal Error: {reason}"))]
    InternalError { reason: &'static str },
    #[snafu(display("Invalid path segment: {val}"))]
    InvalidPathSegmentParameter { val: String },
}
