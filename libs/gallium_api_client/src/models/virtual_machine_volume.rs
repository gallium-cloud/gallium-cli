// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineVolume {
    #[serde(rename = "volumeKubeName")]
    pub volume_kube_name: String,
    #[serde(
        rename = "bus",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bus: Option<Option<String>>,
    #[serde(rename = "bootable")]
    pub bootable: bool,
}

impl VirtualMachineVolume {
    pub fn new(volume_kube_name: String, bootable: bool) -> VirtualMachineVolume {
        VirtualMachineVolume {
            volume_kube_name,
            bus: None,
            bootable,
        }
    }
}
