#[allow(unused)]
pub mod entities;
use crate::api::ApiClient;
use crate::api::errors::ApiClientError;
use crate::api::login_api::entities::{
    GalliumApiSuccessResponse, GalliumLoginRequest, GalliumLoginResponse, GalliumTokenRequest,
    InvalidateTokenRequest,
};
use derive_more::Constructor;
use reqwest::Method;
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
        let response = self
            .api_client
            .request(Method::POST, &["api", "token"])?
            .json(&token_request)
            .header("Gallium-CLI", clap::crate_version!())
            .send()
            .await?;
        self.api_client.deser_response(response).await
    }

    pub async fn login(
        &self,
        login_request: &GalliumLoginRequest,
    ) -> Result<GalliumLoginResponse, ApiClientError> {
        let response = self
            .api_client
            .request(Method::POST, &["api", "login"])?
            .json(&login_request)
            .send()
            .await?;
        self.api_client.deser_response(response).await
    }

    pub async fn invalidate_refresh_token(
        &self,
        invalidate_request: &InvalidateTokenRequest,
    ) -> Result<GalliumApiSuccessResponse, ApiClientError> {
        let response = self
            .api_client
            .request(Method::POST, &["api", "token", "invalidate"])?
            .json(invalidate_request)
            .send()
            .await?;
        self.api_client.deser_response(response).await
    }
}
