use crate::api::errors::ApiClientError;
use crate::args::GlobalArguments;
use crate::helpers::dotfile::read_dotfile;
use crate::task_common::error::TaskError;
use gallium_api_client::apis::configuration::Configuration;
use gallium_api_client::apis::user_api;
use gallium_api_client::apis::{Error, user_api::RefreshAccessTokenError};
use gallium_api_client::models::{GalliumLoginResponse, GalliumTokenRequest};
use reqwest::StatusCode;
use tokio::sync::Mutex;

pub struct AuthSession {
    refresh_token: String,
    org_slug: Option<String>,
    access_token: Mutex<Option<String>>,
}

impl AuthSession {
    pub fn new(refresh_token: String, org_slug: Option<String>) -> Self {
        Self {
            refresh_token,
            org_slug,
            access_token: Mutex::new(None),
        }
    }

    pub async fn load(global_args: &GlobalArguments) -> Result<Self, TaskError> {
        let refresh_token = read_dotfile()
            .await?
            .refresh_tokens
            .remove(global_args.get_api_url())
            .ok_or(TaskError::NotLoggedIn)?;
        Ok(Self::new(refresh_token, global_args.gallium_org.clone()))
    }

    pub async fn refresh(
        &self,
        plain: &Configuration,
    ) -> Result<GalliumLoginResponse, ApiClientError> {
        let mut cached = self.access_token.lock().await;
        let (response, token) = self.fetch_access_token(plain).await?;
        *cached = Some(token);
        Ok(response)
    }

    /// Read the session's access token, fetching one if not present
    // TODO: should this check that the token hasn't expired?
    pub async fn access_token(&self, plain: &Configuration) -> Result<String, ApiClientError> {
        let mut cached = self.access_token.lock().await;
        if let Some(token) = cached.as_ref() {
            return Ok(token.clone());
        }
        let (_, token) = self.fetch_access_token(plain).await?;
        *cached = Some(token.clone());
        Ok(token)
    }

    async fn fetch_access_token(
        &self,
        plain: &Configuration,
    ) -> Result<(GalliumLoginResponse, String), ApiClientError> {
        let response = user_api::refresh_access_token(
            plain,
            GalliumTokenRequest {
                refresh_token: self.refresh_token.clone(),
                org_slug: self.org_slug.clone().map(Some),
            },
        )
        .await
        .map_err(|e| match e {
            Error::<RefreshAccessTokenError>::ResponseError(r)
                if r.status == StatusCode::UNAUTHORIZED =>
            {
                ApiClientError::SessionExpired
            }
            other => other.into(),
        })?;
        let token = response
            .access_token
            .clone()
            .ok_or(ApiClientError::ResponseMissingField {
                field: "accessToken",
            })?;
        Ok((response, token))
    }
}
