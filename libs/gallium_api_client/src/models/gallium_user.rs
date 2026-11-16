// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalliumUser {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    #[serde(rename = "darkMode")]
    pub dark_mode: bool,
}

impl GalliumUser {
    pub fn new(dark_mode: bool) -> GalliumUser {
        GalliumUser {
            email: None,
            name: None,
            flags: None,
            dark_mode,
        }
    }
}
