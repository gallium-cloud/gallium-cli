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
    #[serde(
        rename = "lockfile",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub lockfile: Option<Option<String>>,
    #[serde(
        rename = "startedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub started_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(
        rename = "result",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub result: Option<Option<Box<models::JobResult>>>,
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
