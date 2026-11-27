mod args;
mod browser;
mod password;

use crate::helpers::dotfile::{read_dotfile, write_dotfile};
use crate::task_common::error::TaskError;

use crate::tasks::login::args::LoginMethod;
use crate::tasks::login::browser::browser_login;
use crate::tasks::login::password::password_login;
pub use args::LoginArguments;

pub(crate) async fn login(
    global_args: &crate::args::GlobalArguments,
    args: LoginArguments,
) -> Result<(), TaskError> {
    let login_response = match args.method {
        LoginMethod::Browser => browser_login(global_args, &args).await?,
        LoginMethod::Password => password_login(global_args).await?,
    };

    let refresh_token =
        login_response
            .refresh_token
            .ok_or_else(|| TaskError::ApiResponseMissingField {
                field: "refreshToken",
            })?;

    let mut dotfile = read_dotfile().await?;
    dotfile
        .refresh_tokens
        .insert(global_args.get_api_url().to_string(), refresh_token);
    write_dotfile(&dotfile).await?;

    match login_response.org {
        Some(org) => println!("Logged in to Gallium org: {}", org.name),
        None => println!("Logged in."),
    }
    Ok(())
}
