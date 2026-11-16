use crate::api::ApiClient;
use crate::helpers::cmd::cmd_response::poll_for_cmd_response_type;
use crate::task_common::error::TaskError;
use gallium_api_client::models::CmdSubmitResponse;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NbdServerStartCmdResponse {
    pub host_ip: String,
    pub host_port: u16,
}

const NBD_SERVER_START_CMD_TYPE: &str = "NBD_SERVER_START";

pub async fn poll_for_nbd_response(
    api_client: &ApiClient,
    cmd_submit: &CmdSubmitResponse,
) -> Result<NbdServerStartCmdResponse, TaskError> {
    poll_for_cmd_response_type(api_client, cmd_submit, NBD_SERVER_START_CMD_TYPE).await
}
