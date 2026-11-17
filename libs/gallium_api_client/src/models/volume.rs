// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    #[serde(rename = "kubeName")]
    pub kube_name: String,
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// The total size of the volume
    #[serde(rename = "sizeBytes")]
    pub size_bytes: u64,
    #[serde(
        rename = "volumeType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_type: Option<Option<models::VolumeTypeLabel>>,
    #[serde(rename = "storageClass")]
    pub storage_class: String,
    /// The amount of data that is actually allocated within the volume (actualSize in longhorn terminology). NOTE: Space free-ed by the OS may not be reliably reclaimed.
    #[serde(
        rename = "usedSizeBytes",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub used_size_bytes: Option<Option<u64>>,
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
