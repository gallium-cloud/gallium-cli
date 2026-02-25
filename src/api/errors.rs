use crate::api::common_api::entities::GalliumApiErrorResponse;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum ApiClientError {
    #[snafu(display("API Error: {:?}", error))]
    ApiError { error: GalliumApiErrorResponse },
    #[snafu(transparent)]
    RequestError { source: reqwest::Error },
}
