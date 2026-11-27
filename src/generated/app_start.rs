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
    let response = engines_api::start_application(
        api_client.config(),
        crate::api::path_param(&engine)?,
        crate::api::path_param(&args.name)?,
    )
    .await?;
    wait_for_cmd_completion(&api_client, &response.command_id).await?;
    render_result(
        &response,
        &format!("Started app {} on engine {}.", args.name, engine),
        &args.output,
    )
}
