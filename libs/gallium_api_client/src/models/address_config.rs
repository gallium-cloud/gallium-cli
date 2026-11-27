// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

/// AddressConfig : If dhcp is enabled, ip and gateway must be null. If dhcp is disabled, ip and gateway must be provided and valid.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressConfig {
    /// Whether DHCP is enabled
    #[serde(rename = "dhcp", skip_serializing_if = "Option::is_none")]
    pub dhcp: Option<bool>,
    /// IP address
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Default gateway associated with interface
    #[serde(rename = "gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}

impl AddressConfig {
    /// If dhcp is enabled, ip and gateway must be null. If dhcp is disabled, ip and gateway must be provided and valid.
    pub fn new() -> AddressConfig {
        AddressConfig {
            dhcp: None,
            ip: None,
            gateway: None,
        }
    }
}
