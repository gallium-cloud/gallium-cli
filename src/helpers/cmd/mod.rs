use crate::api::storage_api::entities::{CmdSubmitResponse, SubCommand};
use crate::task_common::error::TaskError;

pub mod cmd_progress;
pub mod cmd_response;

fn find_matching_subcommand<'a>(
    submit_response: &'a CmdSubmitResponse,
    cmd_type: &'static str,
) -> Result<&'a SubCommand, TaskError> {
    let matching_cmds: Vec<_> = submit_response
        .sub_commands
        .iter()
        .filter(|c| c.cmd_type.as_str() == cmd_type)
        .collect();
    if let Some(cmd) = matching_cmds.first()
        && matching_cmds.len() == 1
    {
        Ok(cmd)
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
