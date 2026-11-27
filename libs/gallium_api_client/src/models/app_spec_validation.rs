// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSpecValidation {
    #[serde(rename = "valid")]
    pub valid: bool,
    #[serde(rename = "issues")]
    pub issues: Vec<models::Issue>,
}

impl AppSpecValidation {
    pub fn new(valid: bool, issues: Vec<models::Issue>) -> AppSpecValidation {
        AppSpecValidation { valid, issues }
    }
}
