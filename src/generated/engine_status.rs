// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::helpers::output::{Field, Format, OutputArgs, render_detail};
use crate::task_common::error::TaskError;
use gallium_api_client::apis::engines_api;

#[derive(clap::Args)]
pub struct Args {
    /// Slug of the container engine
    slug: String,
    #[command(flatten)]
    output: OutputArgs,
}

const FIELDS: &[Field] = &[
    Field {
        label: "Online",
        pointer: "/online",
        format: Format::Boolean,
    },
    Field {
        label: "Status",
        pointer: "/status",
        format: Format::Text,
    },
];

pub async fn run(gargs: &GlobalArguments, args: Args) -> Result<(), TaskError> {
    let api_client = gargs.build_authed_api_client().await?;
    let response =
        engines_api::get_engine_status(api_client.config(), crate::api::path_param(&args.slug)?)
            .await?;
    render_detail(&response, FIELDS, &args.output)
}
