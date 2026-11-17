// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsConfiguration {
    #[serde(rename = "nameservers")]
    pub nameservers: Vec<String>,
}

impl DnsConfiguration {
    pub fn new(nameservers: Vec<String>) -> DnsConfiguration {
        DnsConfiguration { nameservers }
    }
}
