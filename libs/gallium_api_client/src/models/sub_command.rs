// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubCommand {
    #[serde(rename = "commandSlug")]
    pub command_slug: String,
    #[serde(rename = "cmdType")]
    pub cmd_type: String,
}

impl SubCommand {
    pub fn new(command_slug: String, cmd_type: String) -> SubCommand {
        SubCommand {
            command_slug,
            cmd_type,
        }
    }
}
