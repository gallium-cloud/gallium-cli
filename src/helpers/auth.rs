use crate::args::GlobalArguments;
use crate::helpers::dotfile::read_dotfile;
use crate::task_common::error::TaskError;
use gallium_api_client::apis::user_api;
use gallium_api_client::models::{GalliumLoginResponse, GalliumTokenRequest};
use snafu::prelude::*;

pub struct AccessToken(pub String);

pub async fn get_login_response_for_saved_credentials(
    global_args: &GlobalArguments,
) -> Result<GalliumLoginResponse, TaskError> {
    let refresh_token = read_dotfile()
        .await?
        .refresh_tokens
        .get(global_args.get_api_url())
        .whatever_context::<_, TaskError>("no refresh token available")?
        .clone();

    let api_client = global_args.build_api_client()?;

    Ok(user_api::refresh_access_token(
        api_client.config(),
        GalliumTokenRequest {
            refresh_token,
            org_slug: global_args.gallium_org.clone().map(Some),
        },
    )
    .await?)
}

impl TryFrom<GalliumLoginResponse> for AccessToken {
    type Error = TaskError;
    fn try_from(value: GalliumLoginResponse) -> Result<Self, Self::Error> {
        value
            .access_token
            .map(AccessToken)
            .ok_or_else(|| TaskError::ApiResponseMissingField {
                field: "accessToken",
            })
    }
}
