// Generated from the Gallium OpenAPI specs. Do not edit.

use super::{ContentType, Error, configuration};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// struct for typed errors of method [`cli_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CliTokenError {
    Status400(models::GalliumApiErrorResponse),
    Status401(models::GalliumApiErrorResponse),
    Status500(models::GalliumApiErrorResponse),
    Status503(models::GalliumApiErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`invalidate_refresh_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InvalidateRefreshTokenError {
    Status401(models::GalliumApiErrorResponse),
    Status500(models::GalliumApiErrorResponse),
    Status503(models::GalliumApiErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`login`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginError {
    Status401(models::GalliumApiErrorResponse),
    Status500(models::GalliumApiErrorResponse),
    Status503(models::GalliumApiErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`refresh_access_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RefreshAccessTokenError {
    Status401(models::GalliumApiErrorResponse),
    Status500(models::GalliumApiErrorResponse),
    Status503(models::GalliumApiErrorResponse),
    UnknownValue(serde_json::Value),
}

pub async fn cli_token(
    configuration: &configuration::Configuration,
    cli_token_request: models::CliTokenRequest,
) -> Result<models::GalliumLoginResponse, Error<CliTokenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_cli_token_request = cli_token_request;

    let uri_str = format!("{}/api/cli/token", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body_cli_token_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::GalliumLoginResponse`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GalliumLoginResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CliTokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn invalidate_refresh_token(
    configuration: &configuration::Configuration,
    invalidate_token_request: models::InvalidateTokenRequest,
) -> Result<models::GalliumApiSuccessResponse, Error<InvalidateRefreshTokenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_invalidate_token_request = invalidate_token_request;

    let uri_str = format!("{}/api/token/invalidate", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body_invalidate_token_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::GalliumApiSuccessResponse`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GalliumApiSuccessResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<InvalidateRefreshTokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn login(
    configuration: &configuration::Configuration,
    gallium_login_request: models::GalliumLoginRequest,
) -> Result<models::GalliumLoginResponse, Error<LoginError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_gallium_login_request = gallium_login_request;

    let uri_str = format!("{}/api/login", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body_gallium_login_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::GalliumLoginResponse`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GalliumLoginResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LoginError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn refresh_access_token(
    configuration: &configuration::Configuration,
    gallium_token_request: models::GalliumTokenRequest,
) -> Result<models::GalliumLoginResponse, Error<RefreshAccessTokenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_gallium_token_request = gallium_token_request;

    let uri_str = format!("{}/api/token", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body_gallium_token_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => {
                return Err(Error::from(serde_json::Error::custom(
                    "Received `text/plain` content type response that cannot be converted to `models::GalliumLoginResponse`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GalliumLoginResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RefreshAccessTokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
