// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeNbdExportRequest {
    #[serde(rename = "csrBase64")]
    pub csr_base64: String,
}

impl VolumeNbdExportRequest {
    pub fn new(csr_base64: String) -> VolumeNbdExportRequest {
        VolumeNbdExportRequest { csr_base64 }
    }
}
