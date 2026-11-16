// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeNbdImportRequest {
    #[serde(rename = "csrBase64")]
    pub csr_base64: String,
    #[serde(rename = "volumeSizeGb")]
    pub volume_size_gb: u32,
    #[serde(rename = "volumeStorageClass")]
    pub volume_storage_class: String,
    #[serde(
        rename = "volume_name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_name: Option<Option<String>>,
    #[serde(
        rename = "volumeDescription",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_description: Option<Option<String>>,
    #[serde(
        rename = "importSourceFileName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub import_source_file_name: Option<Option<String>>,
}

impl VolumeNbdImportRequest {
    pub fn new(
        csr_base64: String,
        volume_size_gb: u32,
        volume_storage_class: String,
    ) -> VolumeNbdImportRequest {
        VolumeNbdImportRequest {
            csr_base64,
            volume_size_gb,
            volume_storage_class,
            volume_name: None,
            volume_description: None,
            import_source_file_name: None,
        }
    }
}
