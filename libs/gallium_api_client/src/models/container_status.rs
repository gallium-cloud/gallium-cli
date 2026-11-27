// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerStatus {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "declared")]
    pub declared: bool,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(
        rename = "lockfile",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub lockfile: Option<Option<String>>,
    #[serde(rename = "restarts")]
    pub restarts: u32,
    #[serde(
        rename = "current",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub current: Option<Option<Box<models::ContainerInstance>>>,
    #[serde(
        rename = "previous",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous: Option<Option<Box<models::ContainerInstance>>>,
}

impl ContainerStatus {
    pub fn new(name: String, declared: bool, image: String, restarts: u32) -> ContainerStatus {
        ContainerStatus {
            name,
            declared,
            image,
            lockfile: None,
            restarts,
            current: None,
            previous: None,
        }
    }
}
