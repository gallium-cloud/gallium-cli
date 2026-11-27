use crate::helpers::output::OutputArgs;
use crate::task_common::error::{
    BackgroundTaskPanickedSnafu, ConfirmationDeclinedSnafu, ConfirmationRequiredSnafu, TaskError,
    UserInputInvalidSnafu,
};
use snafu::{ResultExt, ensure};
use std::io::IsTerminal;

pub async fn confirm(question: &str, yes: bool, output: &OutputArgs) -> Result<(), TaskError> {
    if yes {
        return Ok(());
    }
    ensure!(
        !output.is_json() && std::io::stdin().is_terminal(),
        ConfirmationRequiredSnafu
    );

    let question = question.to_string();
    let confirmed = tokio::task::spawn_blocking(move || {
        dialoguer::Confirm::new()
            .with_prompt(question)
            .default(false)
            .interact()
    })
    .await
    .context(BackgroundTaskPanickedSnafu {
        task: "confirmation prompt",
    })?
    .context(UserInputInvalidSnafu {
        field: "confirmation",
    })?;
    ensure!(confirmed, ConfirmationDeclinedSnafu);
    Ok(())
}
