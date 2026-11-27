use crate::api::{ApiClient, path_param};
use crate::helpers::cmd::find_matching_subcommand;
use crate::task_common::error::TaskError;
use gallium_api_client::apis::command_api_v2_api;
use gallium_api_client::models::{ApiCmdStatus, CmdSubmitResponse};
use serde::de::DeserializeOwned;

pub async fn poll_for_cmd_response<T: DeserializeOwned>(
    api_client: &ApiClient,
    command_slug: String,
) -> Result<T, TaskError> {
    let command_slug = path_param(&command_slug)?;

    loop {
        let details_response =
            command_api_v2_api::get_command_details(api_client.config(), command_slug).await?;

        match details_response.status {
            ApiCmdStatus::Complete => {
                return if let Some(value) = details_response.response_data.flatten() {
                    serde_json::from_value(value).map_err(|e| {
                        TaskError::CommandResponseMissingOrInvalid {
                            cmd_type: details_response.cmd_type,
                            serde_err: Some(e),
                        }
                    })
                } else {
                    Err(TaskError::CommandResponseMissingOrInvalid {
                        cmd_type: details_response.cmd_type,
                        serde_err: None,
                    })
                };
            }
            ApiCmdStatus::Failed => {
                return Err(TaskError::CommandFailure {
                    slug: details_response.command_slug.clone(),
                    cmd_type: details_response.cmd_type.clone(),
                });
            }
            ApiCmdStatus::UnknownValue(status) => {
                return Err(TaskError::CommandStatusUnknown {
                    slug: details_response.command_slug.clone(),
                    status,
                });
            }
            ApiCmdStatus::Pending | ApiCmdStatus::InProgress => {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    }
}

pub async fn poll_for_cmd_response_type<T: DeserializeOwned>(
    api_client: &ApiClient,
    submit_response: &CmdSubmitResponse,
    cmd_type: &'static str,
) -> Result<T, TaskError> {
    let sub_cmd = find_matching_subcommand(submit_response, cmd_type)?;
    poll_for_cmd_response(api_client, sub_cmd.command_slug.clone()).await
}
