use crate::api;
use crate::api::errors::ApiClientError;
use crate::api::login_api::entities::{GalliumLoginRequest, GalliumTokenRequest};
use crate::helpers::dotfile::{read_dotfile, write_dotfile};
use anyhow::anyhow;

pub(crate) async fn login(args: &crate::args::GlobalArguments) {
    let email: String = dialoguer::Input::new()
        .with_prompt("email")
        .interact_text()
        .expect("email address");
    let password: String = dialoguer::Password::new()
        .with_prompt("password")
        .interact()
        .expect("password");

    let mut login_request = GalliumLoginRequest {
        email: email.clone(),
        password: password.clone(),
        otp: None,
        refresh_token: None,
    };

    let login_response;

    loop {
        match api::login_api::post_login(args.get_api_url(), &login_request).await {
            Ok(resp) => {
                if resp.mfa_required {
                    login_request.otp = dialoguer::Input::new()
                        .with_prompt("one-time password from your authenticator")
                        .interact_text()
                        .map(Some)
                        .expect("otp");
                } else {
                    login_response = resp;
                    break;
                }
            }
            Err(ApiClientError::ApiError { error }) => {
                eprintln!(
                    "Error logging in: {}",
                    error.error.unwrap_or("(null)".into())
                );
                return;
            }
            Err(e) => {
                eprintln!("Couldn't connect to API: {:?}", e);
                return;
            }
        };
    }

    let refresh_token = login_response.refresh_token.expect("refresh token");

    let mut dotfile = read_dotfile().await;

    dotfile
        .refresh_tokens
        .insert(args.get_api_url().to_string(), refresh_token);

    write_dotfile(&dotfile).await;
}

pub(crate) async fn logout(args: &crate::args::GlobalArguments) {
    let mut dotfile = read_dotfile().await;

    dotfile.refresh_tokens.remove(args.get_api_url());

    write_dotfile(&dotfile).await;
}

pub(crate) async fn get_access_token(
    api_root_url: &str,
    org_param: &Option<String>,
) -> anyhow::Result<String> {
    let refresh_token = read_dotfile()
        .await
        .refresh_tokens
        .get(api_root_url)
        .ok_or(anyhow::anyhow!("no refresh token available"))?
        .clone();

    api::login_api::post_token(
        api_root_url,
        &GalliumTokenRequest {
            refresh_token,
            org_slug: org_param.clone(),
        },
    )
    .await?
    .access_token
    .ok_or_else(|| anyhow!("API returned null accessToken"))
}
