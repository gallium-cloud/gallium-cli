// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::helpers::output::{Column, Format, OutputArgs, Report, Section, View};
use crate::task_common::error::TaskError;
use gallium_api_client::apis::host_api;

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    output: OutputArgs,
}

static SECTION: Section = Section {
    name: None,
    title: None,
    view: View::List {
        items: "",
        columns: &[
            Column {
                header: "NAME",
                pointer: "/name",
                format: Format::Text,
            },
            Column {
                header: "SLUG",
                pointer: "/slug",
                format: Format::Text,
            },
            Column {
                header: "ONLINE",
                pointer: "/online",
                format: Format::Boolean,
            },
            Column {
                header: "STATUS",
                pointer: "/statusText",
                format: Format::Text,
            },
        ],
    },
};

pub async fn run(global_args: &GlobalArguments, args: Args) -> Result<(), TaskError> {
    let api_client = global_args.build_authed_api_client().await?;
    let mut report = Report::new(&args.output);
    let response = host_api::get_all_hosts(api_client.config(), Some(true)).await?;
    report.add(&SECTION, &response)?;
    report.print()
}
