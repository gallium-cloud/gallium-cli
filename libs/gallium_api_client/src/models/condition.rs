// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "status")]
    pub status: models::ConditionStatus,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: chrono::DateTime<chrono::FixedOffset>,
}

impl Condition {
    pub fn new(
        r#type: String,
        status: models::ConditionStatus,
        reason: String,
        message: String,
        last_transition_time: chrono::DateTime<chrono::FixedOffset>,
    ) -> Condition {
        Condition {
            r#type,
            status,
            reason,
            message,
            last_transition_time,
        }
    }
}
