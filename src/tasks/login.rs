use crate::api::errors::ApiClientError;
use crate::api::login_api::entities::GalliumLoginRequest;
use crate::helpers::dotfile::{read_dotfile, write_dotfile};

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

    let login_api = args.build_api_client().unwrap().login_api();

    let login_response;

    loop {
        match login_api.login(&login_request).await {
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
            Err(ApiClientError::Api { error }) => {
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
