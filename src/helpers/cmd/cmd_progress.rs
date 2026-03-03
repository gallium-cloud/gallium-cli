use crate::api::command_v2_api::CommandApi;
use crate::api::command_v2_api::entities::{
    ApiCmdStatus, CommandApiV2ProgressPutRequest, UpdateCommandProgressPathParams,
};
use crate::api::storage_api::entities::CmdSubmitResponse;
use crate::helpers::cmd::find_matching_subcommand;
use crate::task_common::error::TaskError;

pub struct CommandProgressUpdater {
    command_api: CommandApi,
    cmd_slug: String,
}

impl CommandProgressUpdater {
    pub fn build(
        command_api: CommandApi,
        cmd_submit_response: &CmdSubmitResponse,
        progress_cmd_type: &'static str,
    ) -> Result<Self, TaskError> {
        let matching_cmd = find_matching_subcommand(cmd_submit_response, progress_cmd_type)?;
        Ok(Self {
            command_api,
            cmd_slug: matching_cmd.command_slug.clone(),
        })
    }

    pub async fn complete(self, status: ApiCmdStatus) -> Result<(), TaskError> {
        self.command_api
            .update_command_progress(
                &UpdateCommandProgressPathParams { id: self.cmd_slug },
                &CommandApiV2ProgressPutRequest {
                    progress_current: None,
                    progress_message: None,
                    progress_total: None,
                    status: Some(status),
                },
            )
            .await?;

        Ok(())
    }
}
