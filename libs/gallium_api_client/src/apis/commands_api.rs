// Generated from the Gallium OpenAPI specs. Do not edit.

use super::{ContentType, Error, configuration};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// struct for typed errors of method [`get_command_details`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCommandDetailsError {
    Status500(models::GalliumApiErrorResponse),
    Status503(models::GalliumApiErrorResponse),
    UnknownValue(serde_json::Value),
}

pub async fn get_command_details(
    configuration: &configuration::Configuration,
    command_slug: &str,
) -> Result<models::CommandDetail, Error<GetCommandDetailsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_command_slug = command_slug;

    let uri_str = format!(
        "{}/api/cmd/{commandSlug}",
        configuration.base_path,
        commandSlug = crate::apis::urlencode(p_path_command_slug)
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
                    "Received `text/plain` content type response that cannot be converted to `models::CommandDetail`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CommandDetail`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCommandDetailsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
