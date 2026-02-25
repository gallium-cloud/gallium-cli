#[allow(unused)]
pub mod entities;

use crate::api::common_api::entities::GalliumApiErrorResponse;
use crate::api::errors::ApiClientError;
use crate::api::login_api::entities::{
    GalliumLoginRequest, GalliumLoginResponse, GalliumTokenRequest,
};
use crate::api::ApiClient;
use derive_more::Constructor;
use std::sync::Arc;

#[derive(Constructor)]
pub struct LoginApi {
    api_client: Arc<ApiClient>,
}

impl LoginApi {
    pub async fn refresh_access_token(
        &self,
        token_request: &GalliumTokenRequest,
    ) -> Result<GalliumLoginResponse, ApiClientError> {
        let response = reqwest::Client::new()
            .post(self.api_client.api_url.join("/api/token")?)
            .json(&token_request)
            .header("Gallium-CLI", clap::crate_version!())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<GalliumLoginResponse>().await?)
        } else {
            Err(ApiClientError::Api {
                error: response.json::<GalliumApiErrorResponse>().await?,
            })
        }
    }

    pub async fn login(
        &self,
        login_request: &GalliumLoginRequest,
    ) -> Result<GalliumLoginResponse, ApiClientError> {
        let response = reqwest::Client::new()
            .post(self.api_client.api_url.join("/api/login")?)
            .json(&login_request)
            .header("Gallium-CLI", clap::crate_version!())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<GalliumLoginResponse>().await?)
        } else {
            Err(ApiClientError::Api {
                error: response.json::<GalliumApiErrorResponse>().await?,
            })
        }
    }
}
