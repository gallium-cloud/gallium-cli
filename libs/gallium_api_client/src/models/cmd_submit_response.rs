// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CmdSubmitResponse {
    #[serde(rename = "commandSlug")]
    pub command_slug: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "cmdType")]
    pub cmd_type: String,
    #[serde(rename = "targetEntity", skip_serializing_if = "Option::is_none")]
    pub target_entity: Option<models::CmdEntity>,
    #[serde(rename = "subCommands")]
    pub sub_commands: Vec<models::SubCommand>,
}

impl CmdSubmitResponse {
    pub fn new(
        command_slug: String,
        message: String,
        cmd_type: String,
        sub_commands: Vec<models::SubCommand>,
    ) -> CmdSubmitResponse {
        CmdSubmitResponse {
            command_slug,
            message,
            cmd_type,
            target_entity: None,
            sub_commands,
        }
    }
}
