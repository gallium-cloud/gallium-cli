// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostSummary {
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "online")]
    pub online: bool,
    #[serde(rename = "statusText", skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
    #[serde(rename = "hasContainerEngine")]
    pub has_container_engine: bool,
}

impl HostSummary {
    pub fn new(online: bool, has_container_engine: bool) -> HostSummary {
        HostSummary {
            slug: None,
            name: None,
            description: None,
            online,
            status_text: None,
            has_container_engine,
        }
    }
}
