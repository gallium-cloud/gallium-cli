use crate::api::login_api::entities::{GalliumLoginResponse, GalliumTokenRequest};
use crate::args::GlobalArguments;
use crate::helpers::dotfile::read_dotfile;
use crate::task_common::error::TaskError;
use snafu::prelude::*;

pub(crate) async fn get_login_response_for_saved_credentials(
    global_args: &GlobalArguments,
) -> Result<GalliumLoginResponse, TaskError> {
    let refresh_token = read_dotfile()
        .await
        .refresh_tokens
        .get(global_args.get_api_url())
        .whatever_context::<_, TaskError>("no refresh token available")?
        .clone();

    let login_api = global_args.build_api_client()?.login_api();

    Ok(login_api
        .refresh_access_token(&GalliumTokenRequest {
            refresh_token,
            org_slug: global_args.gallium_org.clone(),
        })
        .await?)
}
