// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeListResponse {
    #[serde(rename = "volumes")]
    pub volumes: Vec<models::Volume>,
}

impl VolumeListResponse {
    pub fn new(volumes: Vec<models::Volume>) -> VolumeListResponse {
        VolumeListResponse { volumes }
    }
}
