use crate::api::login_api::entities::GalliumLoginRequest;
use crate::helpers::dotfile::{read_dotfile, write_dotfile};
use crate::task_common::error::TaskError;

pub(crate) async fn login(args: &crate::args::GlobalArguments) -> Result<(), TaskError> {
    let email: String = dialoguer::Input::new()
        .with_prompt("email")
        .interact_text()
        .map_err(|_| TaskError::UserInputInvalid { field: "email" })?;
    let password: String = dialoguer::Password::new()
        .with_prompt("password")
        .interact()
        .map_err(|_| TaskError::UserInputInvalid { field: "password" })?;

    let mut login_request = GalliumLoginRequest {
        email: email.clone(),
        password: password.clone(),
        otp: None,
        refresh_token: None,
    };

    let login_api = args.build_api_client()?.login_api();

    let login_response;

    loop {
        let resp = login_api.login(&login_request).await?;
        if resp.mfa_required {
            login_request.otp = dialoguer::Input::new()
                .with_prompt("one-time password from your authenticator")
                .interact_text()
                .map(Some)
                .map_err(|_| TaskError::UserInputInvalid { field: "otp" })?;
        } else {
            login_response = resp;
            break;
        }
    }

    let refresh_token =
        login_response
            .refresh_token
            .ok_or_else(|| TaskError::ApiResponseMissingField {
                field: "refreshToken",
            })?;

    let mut dotfile = read_dotfile().await?;

    dotfile
        .refresh_tokens
        .insert(args.get_api_url().to_string(), refresh_token);

    write_dotfile(&dotfile).await?;

    Ok(())
}
