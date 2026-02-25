#[allow(unused)]
pub mod entities;

use crate::api::common_api::entities::GalliumApiErrorResponse;
use crate::api::errors::ApiClientError;
use crate::api::login_api::entities::{
    GalliumLoginRequest, GalliumLoginResponse, GalliumTokenRequest,
};

pub async fn post_token(
    api_root_url: &str,
    token_request: &GalliumTokenRequest,
) -> Result<GalliumLoginResponse, ApiClientError> {
    let response = reqwest::Client::new()
        .post(format!("{}/token", api_root_url))
        .json(&token_request)
        .header("Gallium-CLI", clap::crate_version!())
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.json::<GalliumLoginResponse>().await?)
    } else {
        Err(ApiClientError::ApiError {
            error: response.json::<GalliumApiErrorResponse>().await?,
        })
    }
}

pub async fn post_login(
    api_root_url: &str,
    login_request: &GalliumLoginRequest,
) -> Result<GalliumLoginResponse, ApiClientError> {
    let response = reqwest::Client::new()
        .post(format!("{}/login", api_root_url))
        .json(&login_request)
        .header("Gallium-CLI", clap::crate_version!())
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.json::<GalliumLoginResponse>().await?)
    } else {
        Err(ApiClientError::ApiError {
            error: response.json::<GalliumApiErrorResponse>().await?,
        })
    }
}
