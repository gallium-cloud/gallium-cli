// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolSummary {
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
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "isUserDefault")]
    pub is_user_default: bool,
}

impl DiskPoolSummary {
    pub fn new(kube_name: String, is_default: bool, is_user_default: bool) -> DiskPoolSummary {
        DiskPoolSummary {
            kube_name,
            name: None,
            description: None,
            is_default,
            is_user_default,
        }
    }
}
