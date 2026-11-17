// Generated from the Gallium OpenAPI specs. Do not edit.

use super::{ContentType, Error, configuration};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// struct for typed errors of method [`list_virtual_machines`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVirtualMachinesError {
    Status400(models::AppErrorResponseBody),
    Status500(models::AppErrorResponseBody),
    Status404(models::AppErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// List existing virtual machines
pub async fn list_virtual_machines(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    kube_ns: &str,
) -> Result<models::VirtualMachineListResponse, Error<ListVirtualMachinesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_kube_ns = kube_ns;

    let uri_str = format!(
        "{}/cluster-api/{cluster_id}/vm/{kube_ns}",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id),
        kube_ns = crate::apis::urlencode(p_path_kube_ns)
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
                    "Received `text/plain` content type response that cannot be converted to `models::VirtualMachineListResponse`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::VirtualMachineListResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListVirtualMachinesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
