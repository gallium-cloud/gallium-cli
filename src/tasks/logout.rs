use crate::helpers::dotfile::{read_dotfile, write_dotfile};
use crate::task_common::error::TaskError;

pub(crate) async fn logout(args: &crate::args::GlobalArguments) -> Result<(), TaskError> {
    let mut dotfile = read_dotfile().await?;

    dotfile.refresh_tokens.remove(args.get_api_url());

    write_dotfile(&dotfile).await?;

    Ok(())
}
