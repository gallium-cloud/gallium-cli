// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationStatus {
    #[serde(rename = "phase")]
    pub phase: models::Phase,
    #[serde(rename = "observedAt")]
    pub observed_at: chrono::DateTime<chrono::FixedOffset>,
    #[serde(rename = "stagedChanges")]
    pub staged_changes: models::StagedChanges,
    #[serde(rename = "runningLockfile", skip_serializing_if = "Option::is_none")]
    pub running_lockfile: Option<models::LockfileMetadata>,
    #[serde(rename = "orphanedSecrets")]
    pub orphaned_secrets: Vec<String>,
    #[serde(rename = "volumes")]
    pub volumes: Vec<models::VolumeStatus>,
    #[serde(rename = "conditions")]
    pub conditions: Vec<models::Condition>,
    #[serde(rename = "containers")]
    pub containers: Vec<models::ContainerStatus>,
    #[serde(rename = "jobs", skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<models::JobStatus>>,
}

impl ApplicationStatus {
    pub fn new(
        phase: models::Phase,
        observed_at: chrono::DateTime<chrono::FixedOffset>,
        staged_changes: models::StagedChanges,
        orphaned_secrets: Vec<String>,
        volumes: Vec<models::VolumeStatus>,
        conditions: Vec<models::Condition>,
        containers: Vec<models::ContainerStatus>,
    ) -> ApplicationStatus {
        ApplicationStatus {
            phase,
            observed_at,
            staged_changes,
            running_lockfile: None,
            orphaned_secrets,
            volumes,
            conditions,
            containers,
            jobs: None,
        }
    }
}
