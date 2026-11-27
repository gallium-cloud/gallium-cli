use crate::api::errors::ApiClientError;
use crate::args::GlobalArguments;
use crate::helpers::dotfile::read_dotfile;
use crate::task_common::error::TaskError;
use chrono::{DateTime, Duration, Utc};
use gallium_api_client::apis::configuration::Configuration;
use gallium_api_client::apis::user_api;
use gallium_api_client::apis::{Error, user_api::RefreshAccessTokenError};
use gallium_api_client::models::{GalliumLoginResponse, GalliumTokenRequest};
use jwt_compact::{Claims, UntrustedToken};
use reqwest::StatusCode;
use tokio::sync::Mutex;

pub struct AuthSession {
    refresh_token: String,
    org_slug: Option<String>,
    access_token: Mutex<Option<String>>,
    token_expiry_margin: Duration,
}

impl AuthSession {
    pub async fn load(global_args: &GlobalArguments) -> Result<Self, TaskError> {
        let refresh_token = read_dotfile()
            .await?
            .refresh_tokens
            .remove(global_args.get_api_url())
            .ok_or(TaskError::NotLoggedIn)?;
        Ok(Self {
            refresh_token,
            org_slug: global_args.gallium_org.clone(),
            access_token: Mutex::new(None),
            token_expiry_margin: Duration::seconds(global_args.token_expiry_margin_seconds),
        })
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

    /// Read the session's access token, fetching one if not present or if the cached token is due for renewal
    pub async fn access_token(&self, plain: &Configuration) -> Result<String, ApiClientError> {
        let mut cached = self.access_token.lock().await;
        if let Some(token) = cached.as_ref()
            && token_renewal_due(token, Utc::now(), self.token_expiry_margin) == Some(false)
        {
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
                org_slug: self.org_slug.clone(),
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

fn token_renewal_due(token: &str, now: DateTime<Utc>, margin: Duration) -> Option<bool> {
    let token: Claims<_> = UntrustedToken::new(token)
        .ok()?
        .deserialize_claims_unchecked::<serde_json::Value>()
        .ok()?;

    let expiry = token.expiration?;
    let renewal_due_at = expiry.checked_sub_signed(margin)?;

    Some(now > renewal_due_at)
}
