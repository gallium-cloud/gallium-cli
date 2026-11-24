// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalApplicationList {
    #[serde(rename = "items")]
    pub items: Vec<models::ApplicationListItem>,
}

impl GlobalApplicationList {
    pub fn new(items: Vec<models::ApplicationListItem>) -> GlobalApplicationList {
        GlobalApplicationList { items }
    }
}
