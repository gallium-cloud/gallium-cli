// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::helpers::output::{Column, Format, OutputArgs, Report, Section, View};
use crate::task_common::error::TaskError;
use gallium_api_client::apis::engines_api;

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    output: OutputArgs,
}

static SECTION: Section = Section {
    name: None,
    title: None,
    view: View::List {
        items: "/items",
        columns: &[
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
        ],
    },
};

pub async fn run(global_args: &GlobalArguments, args: Args) -> Result<(), TaskError> {
    let api_client = global_args.build_authed_api_client().await?;
    let mut report = Report::new(&args.output);
    let response =
        engines_api::list_all_applications(api_client.config(), None, None, None).await?;
    report.add(&SECTION, &response)?;
    report.print()
}
