// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerInstance {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "state")]
    pub state: models::ContainerInstanceState,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ready")]
    pub ready: bool,
    #[serde(rename = "startedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "finishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "exitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
}

impl ContainerInstance {
    pub fn new(
        id: String,
        image: String,
        state: models::ContainerInstanceState,
        ready: bool,
    ) -> ContainerInstance {
        ContainerInstance {
            id,
            image,
            state,
            reason: None,
            message: None,
            ready,
            started_at: None,
            finished_at: None,
            exit_code: None,
        }
    }
}
