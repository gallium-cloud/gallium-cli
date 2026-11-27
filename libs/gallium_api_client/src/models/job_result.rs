// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResult {
    #[serde(rename = "job")]
    pub job: String,
    #[serde(rename = "lockfile")]
    pub lockfile: String,
    #[serde(rename = "outcome")]
    pub outcome: models::JobOutcome,
    #[serde(
        rename = "reason",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub reason: Option<Option<String>>,
    #[serde(
        rename = "message",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<Option<String>>,
    #[serde(
        rename = "exitCode",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub exit_code: Option<Option<i32>>,
    #[serde(
        rename = "startedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub started_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
    #[serde(
        rename = "finishedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub finished_at: Option<Option<chrono::DateTime<chrono::FixedOffset>>>,
}

impl JobResult {
    pub fn new(job: String, lockfile: String, outcome: models::JobOutcome) -> JobResult {
        JobResult {
            job,
            lockfile,
            outcome,
            reason: None,
            message: None,
            exit_code: None,
            started_at: None,
            finished_at: None,
        }
    }
}
