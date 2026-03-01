use crate::api::login_api::entities::InvalidateTokenRequest;
use crate::helpers::dotfile::{read_dotfile, write_dotfile};
use crate::task_common::error::TaskError;

pub(crate) async fn logout(args: &crate::args::GlobalArguments) -> Result<(), TaskError> {
    let login_api = args.build_api_client()?.login_api();

    let mut dotfile = read_dotfile().await?;

    if let Some(refresh_token) = dotfile.refresh_tokens.remove(args.get_api_url()) {
        let api_response = login_api
            .invalidate_refresh_token(&InvalidateTokenRequest {
                invalidate_all: false,
                refresh_token,
            })
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
