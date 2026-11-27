use crate::api::{ApiClient, path_param};
use crate::task_common::error::TaskError;
use gallium_api_client::apis::commands_api;
use gallium_api_client::models::ApiCmdStatus;
use indicatif::ProgressBar;
use std::time::Duration;
use tokio::time::Instant;

const POLL_INTERVAL: Duration = Duration::from_secs(1);
const TIMEOUT: Duration = Duration::from_secs(300);
const SPINNER_TICK: Duration = Duration::from_millis(100);

pub async fn wait_for_cmd_completion(
    api_client: &ApiClient,
    command_slug: &str,
) -> Result<(), TaskError> {
    let spinner = ProgressBar::new_spinner().with_message("Waiting for the command to finish");
    spinner.enable_steady_tick(SPINNER_TICK);
    let result = poll_until_complete(api_client, command_slug).await;
    spinner.finish_and_clear();
    result
}

async fn poll_until_complete(api_client: &ApiClient, command_slug: &str) -> Result<(), TaskError> {
    let command_slug = path_param(command_slug)?;
    let deadline = Instant::now() + TIMEOUT;

    loop {
        let details = commands_api::get_command_details(api_client.config(), command_slug).await?;
        let summary = details
            .summary
            .ok_or(TaskError::ApiResponseMissingField { field: "summary" })?;

        match summary.status {
            ApiCmdStatus::Complete => return Ok(()),
            ApiCmdStatus::Failed => {
                return Err(TaskError::CommandFailedWithReason {
                    slug: summary.command_slug,
                    reason: summary
                        .failure_reason
                        .flatten()
                        .unwrap_or_else(|| "unknown failure reason".to_string()),
                });
            }
            ApiCmdStatus::Pending | ApiCmdStatus::InProgress => {
                if Instant::now() >= deadline {
                    return Err(TaskError::CommandTimedOut {
                        slug: summary.command_slug,
                        seconds: TIMEOUT.as_secs(),
                    });
                }
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        }
    }
}
