// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationList2 {
    #[serde(rename = "items")]
    pub items: Vec<models::ApplicationListItem>,
}

impl ApplicationList2 {
    pub fn new(items: Vec<models::ApplicationListItem>) -> ApplicationList2 {
        ApplicationList2 { items }
    }
}
