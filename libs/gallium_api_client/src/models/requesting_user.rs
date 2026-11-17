// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestingUser {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "slug")]
    pub slug: String,
}

impl RequestingUser {
    pub fn new(slug: String) -> RequestingUser {
        RequestingUser { name: None, slug }
    }
}
