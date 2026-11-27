// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::helpers::engine::app_engine;
use crate::helpers::output::{OutputArgs, Report, Section, View};
use crate::task_common::error::TaskError;
use gallium_api_client::apis::applications_api;

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
    /// Number of lines to print. (default: 500)
    #[arg(long, value_name = "LINES")]
    tail: Option<u32>,
    /// Print only entries since the specified timestamp
    #[arg(long, value_name = "TIME")]
    since: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[command(flatten)]
    output: OutputArgs,
}

static SECTION: Section = Section {
    name: None,
    title: None,
    view: View::Lines {
        items: "/events",
        pointer: "/text",
    },
};

pub async fn run(global_args: &GlobalArguments, args: Args) -> Result<(), TaskError> {
    let api_client = global_args.build_authed_api_client().await?;
    let engine = app_engine(
        &api_client,
        args.engine.as_deref(),
        &args.name,
        &args.output,
    )
    .await?;
    let mut report = Report::new(&args.output);
    let response = applications_api::get_container_logs(
        api_client.config(),
        crate::api::path_param(&engine)?,
        crate::api::path_param(&args.name)?,
        crate::api::path_param(&args.container)?,
        args.tail,
        args.since,
    )
    .await?;
    report.add(&SECTION, &response)?;
    report.print()
}
