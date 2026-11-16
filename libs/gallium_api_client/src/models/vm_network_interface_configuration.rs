// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmNetworkInterfaceConfiguration {
    /// MAC address as a string
    #[serde(
        rename = "macAddr",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_addr: Option<Option<String>>,
    #[serde(
        rename = "network",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub network: Option<Option<String>>,
    #[serde(
        rename = "ip",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip: Option<Option<Box<models::AddressConfig>>>,
    #[serde(
        rename = "dns",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dns: Option<Option<Box<models::DnsConfiguration>>>,
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
