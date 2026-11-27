// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::helpers::cmd::cmd_completion::wait_for_cmd_completion;
use crate::helpers::engine::app_engine;
use crate::helpers::output::{OutputArgs, render_result};
use crate::task_common::error::TaskError;
use gallium_api_client::apis::engines_api;

#[derive(clap::Args)]
pub struct Args {
    /// Name of the app
    name: String,
    /// Slug of the container engine
    #[arg(long, value_name = "SLUG")]
    engine: Option<String>,
    /// Name of the container
    #[arg(long)]
    container: String,
    #[command(flatten)]
    output: OutputArgs,
}

pub async fn run(global_args: &GlobalArguments, args: Args) -> Result<(), TaskError> {
    let api_client = global_args.build_authed_api_client().await?;
    let engine = app_engine(
        &api_client,
        args.engine.as_deref(),
        &args.name,
        &args.output,
    )
    .await?;
    let response = engines_api::restart_container(
        api_client.config(),
        crate::api::path_param(&engine)?,
        crate::api::path_param(&args.name)?,
        crate::api::path_param(&args.container)?,
    )
    .await?;
    wait_for_cmd_completion(&api_client, &response.command_id).await?;
    render_result(
        &response,
        &format!(
            "Restarted container {} of app {} on engine {}.",
            args.container, args.name, engine
        ),
        &args.output,
    )
}
