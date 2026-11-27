// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeStatus {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "phase")]
    pub phase: models::VolumePhase,
    #[serde(rename = "sizeBytes", skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    #[serde(rename = "mountedBy")]
    pub mounted_by: Vec<String>,
}

impl VolumeStatus {
    pub fn new(name: String, phase: models::VolumePhase, mounted_by: Vec<String>) -> VolumeStatus {
        VolumeStatus {
            name,
            phase,
            size_bytes: None,
            mounted_by,
        }
    }
}
