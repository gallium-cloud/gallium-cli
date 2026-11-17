// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandSummary {
    #[serde(
        rename = "requestingUser",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub requesting_user: Option<Option<Box<models::RequestingUser>>>,
    #[serde(rename = "subCommands", skip_serializing_if = "Option::is_none")]
    pub sub_commands: Option<Vec<models::SubCommandSummary>>,
    #[serde(rename = "commandSlug")]
    pub command_slug: String,
    #[serde(rename = "issuedAtDisplay")]
    pub issued_at_display: String,
    #[serde(rename = "issuedAt")]
    pub issued_at: String,
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
        rename = "hostSlug",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_slug: Option<Option<String>>,
    #[serde(
        rename = "clusterSlug",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_slug: Option<Option<String>>,
    #[serde(
        rename = "vmSlug",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_slug: Option<Option<String>>,
    #[serde(
        rename = "entityType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_type: Option<Option<String>>,
    #[serde(
        rename = "entityName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_name: Option<Option<String>>,
    #[serde(
        rename = "newState",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_state: Option<Option<String>>,
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
        issued_at_display: String,
        issued_at: String,
        status: models::ApiCmdStatus,
        cmd_type: String,
    ) -> CommandSummary {
        CommandSummary {
            requesting_user: None,
            sub_commands: None,
            command_slug,
            issued_at_display,
            issued_at,
            status,
            cmd_type,
            progress_message: None,
            progress_percent: None,
            host_slug: None,
            cluster_slug: None,
            vm_slug: None,
            entity_type: None,
            entity_name: None,
            new_state: None,
            failure_reason: None,
        }
    }
}
