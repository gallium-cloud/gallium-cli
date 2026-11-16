// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolListResponse {
    #[serde(rename = "pools")]
    pub pools: Vec<models::DiskPoolSummary>,
}

impl DiskPoolListResponse {
    pub fn new(pools: Vec<models::DiskPoolSummary>) -> DiskPoolListResponse {
        DiskPoolListResponse { pools }
    }
}
