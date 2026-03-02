use crate::api::command_v2_api::CommandApi;
use crate::api::storage_api::entities::CmdSubmitResponse;
use crate::helpers::cmd_response::{poll_for_cmd_response, poll_for_cmd_response_type};
use crate::task_common::error::TaskError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NbdServerStartCmdResponse {
    pub host_ip: String,
    pub host_port: u16,
}

const NBD_SERVER_START_CMD_TYPE: &str = "NBD_SERVER_START";

pub async fn poll_for_nbd_response(
    cmd_api: &CommandApi,
    cmd_submit: &CmdSubmitResponse,
) -> Result<NbdServerStartCmdResponse, TaskError> {
    // NBD Server start may be root command or subcommand.
    if cmd_submit.cmd_type.as_str() == NBD_SERVER_START_CMD_TYPE {
        poll_for_cmd_response(cmd_api, cmd_submit.command_slug.clone()).await
    } else {
        poll_for_cmd_response_type(cmd_api, cmd_submit, NBD_SERVER_START_CMD_TYPE).await
    }
}
