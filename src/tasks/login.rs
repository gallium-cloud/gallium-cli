use crate::api;
use crate::api::login_api::entities::GalliumLoginRequest;
use crate::helpers::dotfile::{read_dotfile, write_dotfile};
use std::collections::HashMap;

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

    let mut login_response =
        match api::login_api::post_login(&args.api_root_url, &login_request).await {
            Ok(Ok(login_response)) => login_response,
            Ok(Err(e)) => {
                eprintln!("Error logging in: {}", e.error.unwrap_or("(null)".into()));
                return;
            }
            Err(e) => {
                eprintln!("Couldn't connect to API: {:?}", e);
                return;
            }
        };

    while login_response.mfa_required {
        login_request.otp = dialoguer::Input::new()
            .with_prompt("one-time password from your authenticator")
            .interact_text()
            .map(Some)
            .expect("otp");
        login_response = match api::login_api::post_login(&args.api_root_url, &login_request).await
        {
            Ok(Ok(login_response)) => login_response,
            Ok(Err(e)) => {
                eprintln!("Error logging in: {}", e.error.unwrap_or("(null)".into()));
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
        .insert(args.api_root_url.clone(), refresh_token);

    write_dotfile(&dotfile).await;
}

pub(crate) async fn logout(args: &crate::args::GlobalArguments) {
    let mut dotfile = read_dotfile().await;

    dotfile.refresh_tokens.remove(&args.api_root_url);

    write_dotfile(&dotfile).await;
}

pub(crate) async fn get_access_token(
    api_root_url: &String,
    org_param: &Option<String>,
) -> anyhow::Result<String> {
    let refresh_token = read_dotfile()
        .await
        .refresh_tokens
        .get(api_root_url)
        .ok_or(anyhow::anyhow!("no refresh token available"))?
        .clone();
    let mut params = HashMap::from([("refreshToken", refresh_token)]);
    if let Some(org) = org_param {
        params.insert("orgSlug", org.clone());
    }

    api::login_api::post_token(api_root_url, &params).await
}
