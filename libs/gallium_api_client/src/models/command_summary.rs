// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSummary {
    #[serde(rename = "commandSlug")]
    pub command_slug: String,
    #[serde(rename = "status")]
    pub status: models::ApiCmdStatus,
    #[serde(rename = "cmdType")]
    pub cmd_type: String,
    #[serde(
        rename = "progressMessage",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub progress_message: Option<Option<String>>,
    #[serde(
        rename = "failureReason",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_reason: Option<Option<String>>,
}

impl CommandSummary {
    pub fn new(
        command_slug: String,
        status: models::ApiCmdStatus,
        cmd_type: String,
    ) -> CommandSummary {
        CommandSummary {
            command_slug,
            status,
            cmd_type,
            progress_message: None,
            failure_reason: None,
        }
    }
}
