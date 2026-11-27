// Generated from the Gallium OpenAPI specs. Do not edit.

use super::{ContentType, Error, configuration};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// struct for typed errors of method [`get_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationError {
    Status404(models::ApiError),
    Status409(models::ApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_container_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetContainerLogsError {
    Status400(models::ApiError),
    Status404(models::ApiError),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

pub async fn get_application(
    configuration: &configuration::Configuration,
    engine_slug: &str,
    app_name: &str,
) -> Result<models::Application, Error<GetApplicationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_engine_slug = engine_slug;
    let p_path_app_name = app_name;

    let uri_str = format!(
        "{}/api/engines/{engineSlug}/applications/{appName}",
        configuration.base_path,
        engineSlug = crate::apis::urlencode(p_path_engine_slug),
        appName = crate::apis::urlencode(p_path_app_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

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
                    "Received `text/plain` content type response that cannot be converted to `models::Application`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::Application`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_container_logs(
    configuration: &configuration::Configuration,
    engine_slug: &str,
    app_name: &str,
    container_name: &str,
    tail: Option<u32>,
    since: Option<chrono::DateTime<chrono::FixedOffset>>,
) -> Result<models::ContainerLogs, Error<GetContainerLogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_engine_slug = engine_slug;
    let p_path_app_name = app_name;
    let p_path_container_name = container_name;
    let p_query_tail = tail;
    let p_query_since = since;

    let uri_str = format!(
        "{}/api/engines/{engineSlug}/applications/{appName}/containers/{containerName}/logs",
        configuration.base_path,
        engineSlug = crate::apis::urlencode(p_path_engine_slug),
        appName = crate::apis::urlencode(p_path_app_name),
        containerName = crate::apis::urlencode(p_path_container_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_tail {
        req_builder = req_builder.query(&[("tail", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_since {
        req_builder = req_builder.query(&[("since", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

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
                    "Received `text/plain` content type response that cannot be converted to `models::ContainerLogs`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ContainerLogs`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetContainerLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
