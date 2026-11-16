use crate::helpers::dotfile::{read_dotfile, write_dotfile};
use crate::task_common::error::{TaskError, UserInputInvalidSnafu};
use gallium_api_client::apis::user_api;
use gallium_api_client::models::GalliumLoginRequest;
use snafu::ResultExt;

pub(crate) async fn login(args: &crate::args::GlobalArguments) -> Result<(), TaskError> {
    let email: String = dialoguer::Input::new()
        .with_prompt("email")
        .interact_text()
        .context(UserInputInvalidSnafu { field: "email" })?;
    let password: String = dialoguer::Password::new()
        .with_prompt("password")
        .interact()
        .context(UserInputInvalidSnafu { field: "password" })?;

    let mut login_request = GalliumLoginRequest {
        email: email.clone(),
        password: password.clone(),
        otp: None,
    };

    let api_client = args.build_api_client()?;

    let login_response;

    loop {
        let resp = user_api::login(api_client.config(), login_request.clone()).await?;
        if resp.mfa_required {
            let otp = dialoguer::Input::new()
                .with_prompt("one-time password from your authenticator")
                .interact_text()
                .context(UserInputInvalidSnafu { field: "otp" })?;
            login_request.otp = Some(Some(otp));
        } else {
            login_response = resp;
            break;
        }
    }

    let refresh_token = login_response.refresh_token.flatten().ok_or_else(|| {
        TaskError::ApiResponseMissingField {
            field: "refreshToken",
        }
    })?;

    let mut dotfile = read_dotfile().await?;

    dotfile
        .refresh_tokens
        .insert(args.get_api_url().to_string(), refresh_token);

    write_dotfile(&dotfile).await?;

    Ok(())
}
