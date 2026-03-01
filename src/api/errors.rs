use crate::api::common_api::entities::GalliumApiErrorResponse;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum ApiClientError {
    #[snafu(display("API Error: {:?}", error))]
    Api { error: GalliumApiErrorResponse },
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
