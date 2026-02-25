use crate::api::login_api::entities::{GalliumLoginResponse, GalliumTokenRequest};
use crate::api::login_api::LoginApi;
use crate::helpers::dotfile::read_dotfile;

pub(crate) async fn get_login_response_for_saved_credentials(
    api_root_url: &str,
    org_param: &Option<String>,
) -> anyhow::Result<GalliumLoginResponse> {
    let refresh_token = read_dotfile()
        .await
        .refresh_tokens
        .get(api_root_url)
        .ok_or(anyhow::anyhow!("no refresh token available"))?
        .clone();

    Ok(LoginApi::refresh_access_token(
        api_root_url,
        &GalliumTokenRequest {
            refresh_token,
            org_slug: org_param.clone(),
        },
    )
    .await?)
}
