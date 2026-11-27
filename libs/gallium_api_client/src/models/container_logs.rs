// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerLogs {
    #[serde(rename = "observedAt")]
    pub observed_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "container")]
    pub container: String,
    #[serde(
        rename = "instance",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub instance: Option<Option<String>>,
    #[serde(rename = "events")]
    pub events: Vec<models::LogEvent>,
}

impl ContainerLogs {
    pub fn new(
        observed_at: chrono::DateTime<chrono::FixedOffset>,
        container: String,
        events: Vec<models::LogEvent>,
    ) -> ContainerLogs {
        ContainerLogs {
            observed_at,
            container,
            instance: None,
            events,
        }
    }
}
