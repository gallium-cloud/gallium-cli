use crate::api::command_v2_api::CommandApi;
use crate::api::command_v2_api::entities::{ApiCmdStatus, GetCommandDetailsPathParams};
use crate::api::storage_api::entities::CmdSubmitResponse;
use crate::task_common::error::TaskError;
use serde::de::DeserializeOwned;

pub async fn poll_for_cmd_response<T: DeserializeOwned>(
    api: &CommandApi,
    command_slug: String,
) -> Result<T, TaskError> {
    let params = GetCommandDetailsPathParams { id: command_slug };

    loop {
        let details_response = api.get_command_details(&params).await?;

        match details_response.status {
            ApiCmdStatus::COMPLETE => {
                return if let Some(value) = details_response.response_data {
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
            ApiCmdStatus::FAILED => {
                return Err(TaskError::CommandFailure {
                    slug: details_response.command_slug.clone(),
                    cmd_type: details_response.cmd_type.clone(),
                });
            }
            ApiCmdStatus::PENDING | ApiCmdStatus::INPROGRESS => {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                println!("polling");
            }
        }
    }
}

pub async fn poll_for_cmd_response_type<T: DeserializeOwned>(
    api: &CommandApi,
    submit_response: &CmdSubmitResponse,
    cmd_type: &'static str,
) -> Result<T, TaskError> {
    let matching_cmds: Vec<_> = submit_response
        .sub_commands
        .iter()
        .filter(|c| c.cmd_type.as_str() == cmd_type)
        .collect();
    if let Some(cmd) = matching_cmds.first()
        && matching_cmds.len() == 1
    {
        poll_for_cmd_response(api, cmd.command_slug.clone()).await
    } else {
        Err(TaskError::InvalidStateForCommand {
            command: cmd_type,
            reason: format!(
                "Expected exactly one matching sub-command, found {}",
                matching_cmds.len()
            ),
        })
    }
}
