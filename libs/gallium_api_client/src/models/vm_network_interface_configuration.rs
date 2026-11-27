// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmNetworkInterfaceConfiguration {
    /// MAC address as a string
    #[serde(rename = "macAddr", skip_serializing_if = "Option::is_none")]
    pub mac_addr: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<models::AddressConfig>,
    #[serde(rename = "dns", skip_serializing_if = "Option::is_none")]
    pub dns: Option<models::DnsConfiguration>,
}

impl VmNetworkInterfaceConfiguration {
    pub fn new() -> VmNetworkInterfaceConfiguration {
        VmNetworkInterfaceConfiguration {
            mac_addr: None,
            network: None,
            ip: None,
            dns: None,
        }
    }
}
