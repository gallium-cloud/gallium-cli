// Generated from the Gallium OpenAPI specs. Do not edit.

use super::{ContentType, Error, configuration};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// struct for typed errors of method [`export_nbd_volume`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExportNbdVolumeError {
    Status400(models::AppErrorResponseBody),
    Status500(models::AppErrorResponseBody),
    Status404(models::AppErrorResponseBody),
    Status415(String),
    Status422(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`import_nbd_volume`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportNbdVolumeError {
    Status400(models::AppErrorResponseBody),
    Status500(models::AppErrorResponseBody),
    Status404(models::AppErrorResponseBody),
    Status415(String),
    Status422(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_volumes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVolumesError {
    Status400(models::AppErrorResponseBody),
    Status500(models::AppErrorResponseBody),
    Status404(models::AppErrorResponseBody),
    UnknownValue(serde_json::Value),
}

/// Command to export a volume via NBD
pub async fn export_nbd_volume(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    kube_ns: &str,
    kube_name: &str,
    volume_nbd_export_request: models::VolumeNbdExportRequest,
) -> Result<models::CmdSubmitResponse, Error<ExportNbdVolumeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_kube_ns = kube_ns;
    let p_path_kube_name = kube_name;
    let p_body_volume_nbd_export_request = volume_nbd_export_request;

    let uri_str = format!(
        "{}/cluster-api/{cluster_id}/volume/{kube_ns}/{kube_name}/nbd/export",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id),
        kube_ns = crate::apis::urlencode(p_path_kube_ns),
        kube_name = crate::apis::urlencode(p_path_kube_name)
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
    req_builder = req_builder.json(&p_body_volume_nbd_export_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CmdSubmitResponse`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CmdSubmitResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ExportNbdVolumeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Command to import a volume via NBD
pub async fn import_nbd_volume(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    kube_ns: &str,
    volume_nbd_import_request: models::VolumeNbdImportRequest,
) -> Result<models::CmdSubmitResponse, Error<ImportNbdVolumeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_kube_ns = kube_ns;
    let p_body_volume_nbd_import_request = volume_nbd_import_request;

    let uri_str = format!(
        "{}/cluster-api/{cluster_id}/volume/{kube_ns}/nbd/import",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id),
        kube_ns = crate::apis::urlencode(p_path_kube_ns)
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
    req_builder = req_builder.json(&p_body_volume_nbd_import_request);

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
                    "Received `text/plain` content type response that cannot be converted to `models::CmdSubmitResponse`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CmdSubmitResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ImportNbdVolumeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List existing volumes
pub async fn list_volumes(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    kube_ns: &str,
    pool_kind: Option<&str>,
) -> Result<models::VolumeListResponse, Error<ListVolumesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_kube_ns = kube_ns;
    let p_query_pool_kind = pool_kind;

    let uri_str = format!(
        "{}/cluster-api/{cluster_id}/volume/{kube_ns}",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id),
        kube_ns = crate::apis::urlencode(p_path_kube_ns)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_pool_kind {
        req_builder = req_builder.query(&[("poolKind", &param_value.to_string())]);
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
                    "Received `text/plain` content type response that cannot be converted to `models::VolumeListResponse`",
                )));
            }
            ContentType::Unsupported(unknown_type) => {
                return Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::VolumeListResponse`"
                ))));
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListVolumesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
