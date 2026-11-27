// Generated from the Gallium OpenAPI specs. Do not edit.

use super::{ContentType, Error, configuration};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// struct for typed errors of method [`create_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationError {
    Status400(models::AppErrorResponseBody),
    Status500(models::AppErrorResponseBody),
    Status404(models::AppErrorResponseBody),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deploy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeployError {
    Status400(models::AppErrorResponseBody),
    Status500(models::AppErrorResponseBody),
    Status404(models::AppErrorResponseBody),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_engine_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEngineStatusError {
    Status400(models::AppErrorResponseBody),
    Status500(models::AppErrorResponseBody),
    Status404(models::AppErrorResponseBody),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_all_applications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAllApplicationsError {
    Status400(models::AppErrorResponseBody),
    Status500(models::AppErrorResponseBody),
    Status404(models::AppErrorResponseBody),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

pub async fn create_application(
    configuration: &configuration::Configuration,
    engine_slug: &str,
    app_name: &str,
    body: &str,
    if_match: Option<&str>,
) -> Result<models::SagaSubmitted, Error<CreateApplicationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_engine_slug = engine_slug;
    let p_path_app_name = app_name;
    let p_body_body = body;
    let p_header_if_match = if_match;

    let uri_str = format!(
        "{}/api/ce/{engineSlug}/applications/{appName}",
        configuration.base_path,
        engineSlug = crate::apis::urlencode(p_path_engine_slug),
        appName = crate::apis::urlencode(p_path_app_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_if_match {
        req_builder = req_builder.header("If-Match", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder
        .header(reqwest::header::CONTENT_TYPE, "application/yaml")
        .body(p_body_body.to_string());

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
                    "Received `text/plain` content type response that cannot be converted to `models::SagaSubmitted`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SagaSubmitted`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn deploy(
    configuration: &configuration::Configuration,
    engine_slug: &str,
    app_name: &str,
) -> Result<models::DeployResponse, Error<DeployError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_engine_slug = engine_slug;
    let p_path_app_name = app_name;

    let uri_str = format!(
        "{}/api/ce/{engineSlug}/applications/{appName}/deploy",
        configuration.base_path,
        engineSlug = crate::apis::urlencode(p_path_engine_slug),
        appName = crate::apis::urlencode(p_path_app_name)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

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
                    "Received `text/plain` content type response that cannot be converted to `models::DeployResponse`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::DeployResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeployError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_engine_status(
    configuration: &configuration::Configuration,
    engine_slug: &str,
) -> Result<models::EngineStatus, Error<GetEngineStatusError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_engine_slug = engine_slug;

    let uri_str = format!(
        "{}/api/engines/{engineSlug}/status",
        configuration.base_path,
        engineSlug = crate::apis::urlencode(p_path_engine_slug)
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
                    "Received `text/plain` content type response that cannot be converted to `models::EngineStatus`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::EngineStatus`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEngineStatusError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn list_all_applications(
    configuration: &configuration::Configuration,
    engine_slug: Option<&str>,
    phase: Option<models::Phase>,
    application_id: Option<&str>,
) -> Result<models::GlobalApplicationList, Error<ListAllApplicationsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_engine_slug = engine_slug;
    let p_query_phase = phase;
    let p_query_application_id = application_id;

    let uri_str = format!("{}/api/ce/applications", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_engine_slug {
        req_builder = req_builder.query(&[("engineSlug", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_phase {
        req_builder = req_builder.query(&[("phase", &serde_json::to_string(param_value)?)]);
    }
    if let Some(ref param_value) = p_query_application_id {
        req_builder = req_builder.query(&[("applicationId", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::GlobalApplicationList`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GlobalApplicationList`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListAllApplicationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
