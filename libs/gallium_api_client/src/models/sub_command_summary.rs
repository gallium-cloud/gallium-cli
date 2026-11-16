// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubCommandSummary {
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
    /// Where applicable, a percentage value indicating command progress.
    #[serde(
        rename = "progressPercent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub progress_percent: Option<Option<f64>>,
    #[serde(
        rename = "entityName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_name: Option<Option<String>>,
    #[serde(
        rename = "entityType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_type: Option<Option<String>>,
    #[serde(
        rename = "failureReason",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_reason: Option<Option<String>>,
    /// Represents the sequence number within the overall command. IMPORTANT: Because commands execute in parallel when their dependants are ready, this is really mostly about vibes.
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: i32,
}

impl SubCommandSummary {
    pub fn new(
        command_slug: String,
        status: models::ApiCmdStatus,
        cmd_type: String,
        sequence_number: i32,
    ) -> SubCommandSummary {
        SubCommandSummary {
            command_slug,
            status,
            cmd_type,
            progress_message: None,
            progress_percent: None,
            entity_name: None,
            entity_type: None,
            failure_reason: None,
            sequence_number,
        }
    }
}
