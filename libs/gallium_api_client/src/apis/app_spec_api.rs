// Generated from the Gallium OpenAPI specs. Do not edit.

use super::{ContentType, Error, configuration};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// struct for typed errors of method [`validate_app_spec`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateAppSpecError {
    Status400(),
    Status500(models::AppErrorResponseBody),
    Status404(models::AppErrorResponseBody),
    Status415(),
    DefaultResponse(models::ApiError),
    UnknownValue(serde_json::Value),
}

pub async fn validate_app_spec(
    configuration: &configuration::Configuration,
    body: &str,
) -> Result<models::AppSpecValidation, Error<ValidateAppSpecError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_body = body;

    let uri_str = format!("{}/api/appspec/validate", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
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
                    "Received `text/plain` content type response that cannot be converted to `models::AppSpecValidation`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::AppSpecValidation`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ValidateAppSpecError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
