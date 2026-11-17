use crate::api::{ApiClient, path_param};
use crate::helpers::cmd::find_matching_subcommand;
use crate::task_common::error::TaskError;
use gallium_api_client::apis::command_api_v2_api;
use gallium_api_client::models::{ApiCmdStatus, CmdSubmitResponse, CommandApiV2ProgressPutRequest};
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct CommandProgressUpdater {
    api_client: Arc<ApiClient>,
    cmd_slug: String,
    tx: Sender<CommandApiV2ProgressPutRequest>,
}

impl CommandProgressUpdater {
    pub fn build_and_spawn(
        api_client: Arc<ApiClient>,
        cmd_submit_response: &CmdSubmitResponse,
        progress_cmd_type: &'static str,
    ) -> Result<Self, TaskError> {
        let matching_cmd = find_matching_subcommand(cmd_submit_response, progress_cmd_type)?;

        let (tx, rx) = tokio::sync::mpsc::channel(1);

        tokio::spawn(send_progress(
            api_client.clone(),
            matching_cmd.command_slug.clone(),
            rx,
        ));

        Ok(Self {
            api_client,
            cmd_slug: matching_cmd.command_slug.clone(),
            tx,
        })
    }

    pub fn update_progress(&self, current: f64, total: f64) {
        self.tx
            .try_send(CommandApiV2ProgressPutRequest {
                progress_current: Some(Some(current)),
                progress_total: Some(Some(total)),
                progress_message: None,
                status: None,
            })
            .ok();
    }

    pub async fn complete(self, status: ApiCmdStatus) -> Result<(), TaskError> {
        command_api_v2_api::update_command_progress(
            self.api_client.config(),
            path_param(&self.cmd_slug)?,
            CommandApiV2ProgressPutRequest {
                progress_current: None,
                progress_message: None,
                progress_total: None,
                status: Some(Some(status)),
            },
        )
        .await?;

        Ok(())
    }
}

async fn send_progress(
    api_client: Arc<ApiClient>,
    cmd_slug: String,
    mut rx: Receiver<CommandApiV2ProgressPutRequest>,
) {
    let Ok(cmd_slug) = path_param(&cmd_slug) else {
        return;
    };
    while let Some(msg) = rx.recv().await {
        command_api_v2_api::update_command_progress(api_client.config(), cmd_slug, msg)
            .await
            .ok();
    }
}
