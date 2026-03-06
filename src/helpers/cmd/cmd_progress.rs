use crate::api::command_v2_api::CommandApi;
use crate::api::command_v2_api::entities::{
    ApiCmdStatus, CommandApiV2ProgressPutRequest, UpdateCommandProgressPathParams,
};
use crate::api::storage_api::entities::CmdSubmitResponse;
use crate::helpers::cmd::find_matching_subcommand;
use crate::task_common::error::TaskError;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct CommandProgressUpdater {
    command_api: CommandApi,
    cmd_slug: String,
    tx: Sender<CommandApiV2ProgressPutRequest>,
}

impl CommandProgressUpdater {
    pub fn build_and_spawn(
        command_api: CommandApi,
        cmd_submit_response: &CmdSubmitResponse,
        progress_cmd_type: &'static str,
    ) -> Result<Self, TaskError> {
        let matching_cmd = find_matching_subcommand(cmd_submit_response, progress_cmd_type)?;

        let (tx, rx) = tokio::sync::mpsc::channel(1);

        tokio::spawn(send_progress(
            command_api.clone(),
            UpdateCommandProgressPathParams {
                id: matching_cmd.command_slug.clone(),
            },
            rx,
        ));

        Ok(Self {
            command_api,
            cmd_slug: matching_cmd.command_slug.clone(),
            tx,
        })
    }

    pub fn update_progress(&self, current: f64, total: f64) {
        self.tx
            .try_send(CommandApiV2ProgressPutRequest {
                progress_current: Some(current),
                progress_total: Some(total),
                progress_message: None,
                status: None,
            })
            .ok();
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

async fn send_progress(
    command_api: CommandApi,
    path: UpdateCommandProgressPathParams,
    mut rx: Receiver<CommandApiV2ProgressPutRequest>,
) {
    while let Some(msg) = rx.recv().await {
        command_api.update_command_progress(&path, &msg).await.ok();
    }
}
