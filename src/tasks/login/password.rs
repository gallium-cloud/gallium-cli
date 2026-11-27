use crate::args::GlobalArguments;
use crate::task_common::error::{TaskError, UserInputInvalidSnafu};
use gallium_api_client::apis::user_api;
use gallium_api_client::models::{GalliumLoginRequest, GalliumLoginResponse};
use snafu::ResultExt;

pub async fn password_login(args: &GlobalArguments) -> Result<GalliumLoginResponse, TaskError> {
    let email: String = dialoguer::Input::new()
        .with_prompt("email")
        .interact_text()
        .context(UserInputInvalidSnafu { field: "email" })?;
    let password: String = dialoguer::Password::new()
        .with_prompt("password")
        .interact()
        .context(UserInputInvalidSnafu { field: "password" })?;

    let mut login_request = GalliumLoginRequest {
        email,
        password,
        otp: None,
    };

    let api_client = args.build_unauthed_api_client()?;

    loop {
        let resp = user_api::login(api_client.config(), login_request.clone()).await?;
        if resp.mfa_required {
            let otp = dialoguer::Input::new()
                .with_prompt("one-time password from your authenticator")
                .interact_text()
                .context(UserInputInvalidSnafu { field: "otp" })?;
            login_request.otp = Some(otp);
        } else {
            return Ok(resp);
        }
    }
}
