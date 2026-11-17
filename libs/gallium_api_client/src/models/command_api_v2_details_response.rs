// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandApiV2DetailsResponse {
    /// An identifier that uniquely identifies an event, user, resource object, etc in Gallium's platform.
    #[serde(rename = "commandSlug")]
    pub command_slug: String,
    #[serde(rename = "status")]
    pub status: models::ApiCmdStatus,
    #[serde(rename = "cmdType")]
    pub cmd_type: String,
    #[serde(
        rename = "responseData",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_data: Option<Option<serde_json::Value>>,
}

impl CommandApiV2DetailsResponse {
    pub fn new(
        command_slug: String,
        status: models::ApiCmdStatus,
        cmd_type: String,
    ) -> CommandApiV2DetailsResponse {
        CommandApiV2DetailsResponse {
            command_slug,
            status,
            cmd_type,
            response_data: None,
        }
    }
}
