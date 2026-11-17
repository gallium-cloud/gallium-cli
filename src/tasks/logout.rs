use crate::helpers::dotfile::{read_dotfile, write_dotfile};
use crate::task_common::error::TaskError;
use gallium_api_client::apis::user_api;
use gallium_api_client::models::InvalidateTokenRequest;

pub(crate) async fn logout(args: &crate::args::GlobalArguments) -> Result<(), TaskError> {
    let api_client = args.build_unauthed_api_client()?;

    let mut dotfile = read_dotfile().await?;

    if let Some(refresh_token) = dotfile.refresh_tokens.remove(args.get_api_url()) {
        let api_response = user_api::invalidate_refresh_token(
            api_client.config(),
            InvalidateTokenRequest {
                invalidate_all: false,
                refresh_token,
            },
        )
        .await?;

        println!(
            "Logout response: {}",
            api_response.message.as_deref().unwrap_or("")
        );

        write_dotfile(&dotfile).await?;

        println!("Credentials removed from dotfile");

        Ok(())
    } else {
        Err(TaskError::InvalidStateForCommand {
            command: "logout",
            reason: "Not logged in.".into(),
        })
    }
}
