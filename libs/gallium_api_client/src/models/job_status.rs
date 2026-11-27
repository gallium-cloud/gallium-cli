// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStatus {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "when")]
    pub when: models::JobTrigger,
    #[serde(rename = "phase")]
    pub phase: models::JobPhase,
    #[serde(rename = "lockfile", skip_serializing_if = "Option::is_none")]
    pub lockfile: Option<String>,
    #[serde(rename = "startedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<models::JobResult>,
}

impl JobStatus {
    pub fn new(name: String, when: models::JobTrigger, phase: models::JobPhase) -> JobStatus {
        JobStatus {
            name,
            when,
            phase,
            lockfile: None,
            started_at: None,
            result: None,
        }
    }
}
