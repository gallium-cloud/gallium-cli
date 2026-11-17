// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CmdEntity {
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    #[serde(rename = "kube_ns")]
    pub kube_ns: String,
    #[serde(rename = "kube_name")]
    pub kube_name: String,
}

impl CmdEntity {
    pub fn new(entity_type: String, kube_ns: String, kube_name: String) -> CmdEntity {
        CmdEntity {
            entity_type,
            kube_ns,
            kube_name,
        }
    }
}
