// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandDetail {
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<Box<models::CommandSummary>>,
}

impl CommandDetail {
    pub fn new() -> CommandDetail {
        CommandDetail { summary: None }
    }
}
