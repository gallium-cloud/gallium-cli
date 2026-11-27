// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    #[serde(rename = "kubeName")]
    pub kube_name: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The total size of the volume
    #[serde(rename = "sizeBytes")]
    pub size_bytes: u64,
    #[serde(rename = "volumeType", skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<models::VolumeTypeLabel>,
    #[serde(rename = "storageClass")]
    pub storage_class: String,
    /// The amount of data that is actually allocated within the volume (actualSize in longhorn terminology). NOTE: Space free-ed by the OS may not be reliably reclaimed.
    #[serde(rename = "usedSizeBytes", skip_serializing_if = "Option::is_none")]
    pub used_size_bytes: Option<u64>,
}

impl Volume {
    pub fn new(kube_name: String, size_bytes: u64, storage_class: String) -> Volume {
        Volume {
            kube_name,
            name: None,
            description: None,
            size_bytes,
            volume_type: None,
            storage_class,
            used_size_bytes: None,
        }
    }
}
