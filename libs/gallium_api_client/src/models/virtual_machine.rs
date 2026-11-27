// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(rename = "kubeName")]
    pub kube_name: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "interfaces")]
    pub interfaces: Vec<models::VmNetworkInterfaceConfiguration>,
    #[serde(rename = "volumes")]
    pub volumes: Vec<models::VirtualMachineVolume>,
    #[serde(rename = "status")]
    pub status: models::VirtualMachineStatus,
}

impl VirtualMachine {
    pub fn new(
        kube_name: String,
        interfaces: Vec<models::VmNetworkInterfaceConfiguration>,
        volumes: Vec<models::VirtualMachineVolume>,
        status: models::VirtualMachineStatus,
    ) -> VirtualMachine {
        VirtualMachine {
            kube_name,
            name: None,
            description: None,
            interfaces,
            volumes,
            status,
        }
    }
}
