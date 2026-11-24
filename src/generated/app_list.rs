// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::helpers::output::{Column, Format, OutputArgs, render_list};
use crate::task_common::error::TaskError;
use gallium_api_client::apis::engines_api;

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    output: OutputArgs,
}

const COLUMNS: &[Column] = &[
    Column {
        header: "NAME",
        pointer: "/name",
        format: Format::Text,
    },
    Column {
        header: "PHASE",
        pointer: "/phase",
        format: Format::Text,
    },
    Column {
        header: "SYNCED",
        pointer: "/syncedAt",
        format: Format::Age,
    },
    Column {
        header: "ENGINE",
        pointer: "/engineName",
        format: Format::Text,
    },
    Column {
        header: "ENGINE ONLINE",
        pointer: "/engineOnline",
        format: Format::Boolean,
    },
    Column {
        header: "ENGINE SLUG",
        pointer: "/engineSlug",
        format: Format::Text,
    },
];

pub async fn run(global_args: &GlobalArguments, args: Args) -> Result<(), TaskError> {
    let api_client = global_args.build_authed_api_client().await?;
    let response =
        engines_api::list_all_applications(api_client.config(), None, None, None).await?;
    render_list(&response, "/items", COLUMNS, &args.output)
}
