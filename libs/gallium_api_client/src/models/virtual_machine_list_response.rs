// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineListResponse {
    #[serde(rename = "items")]
    pub items: Vec<models::VirtualMachine>,
}

impl VirtualMachineListResponse {
    pub fn new(items: Vec<models::VirtualMachine>) -> VirtualMachineListResponse {
        VirtualMachineListResponse { items }
    }
}
