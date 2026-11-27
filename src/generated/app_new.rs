// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::helpers::cmd::cmd_completion::wait_for_cmd_completion;
use crate::helpers::document::read_document;
use crate::helpers::output::{OutputArgs, render_result};
use crate::helpers::yaml;
use crate::task_common::error::TaskError;
use gallium_api_client::apis::engines_api;

#[derive(clap::Args)]
pub struct Args {
    /// Name of the new app
    name: String,
    /// Identifier of the container engine where the app should be created
    #[arg(long, value_name = "SLUG")]
    engine: String,
    #[command(flatten)]
    body: BodyArgs,
    #[command(flatten)]
    output: OutputArgs,
}

#[derive(clap::Args)]
#[group(required = true, multiple = false)]
struct BodyArgs {
    /// Container image to deploy into the new app
    #[arg(long)]
    image: Option<String>,
    /// AppSpec that describes the new app
    #[arg(short = 'f', long, value_name = "PATH")]
    file: Option<std::path::PathBuf>,
}

pub async fn run(global_args: &GlobalArguments, args: Args) -> Result<(), TaskError> {
    let api_client = global_args.build_authed_api_client().await?;
    let body = if let Some(file) = &args.body.file {
        read_document(file).await?
    } else if let Some(image) = &args.body.image {
        yaml::document(yaml::map([
            ("version", yaml::string("v1")),
            (
                "containers",
                yaml::map([("app", yaml::map([("image", yaml::string(image))]))]),
            ),
        ]))?
    } else {
        return Err(TaskError::RequiredParameterMissing {
            reason: "--image or --file",
        });
    };
    let response = engines_api::create_application(
        api_client.config(),
        crate::api::path_param(&args.engine)?,
        crate::api::path_param(&args.name)?,
        &body,
        None,
    )
    .await?;
    wait_for_cmd_completion(&api_client, &response.command_id).await?;
    render_result(
        &response,
        &format!("Created app {} on engine {}.", args.name, args.engine),
        &args.output,
    )
}
