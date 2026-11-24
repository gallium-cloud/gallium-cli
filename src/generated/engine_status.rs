// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::helpers::output::{Column, Field, Format, OutputArgs, Report, Section, View};
use crate::task_common::error::TaskError;
use gallium_api_client::apis::engines_api;

#[derive(clap::Args)]
pub struct Args {
    /// Slug of the container engine
    slug: String,
    #[command(flatten)]
    output: OutputArgs,
}

static STATUS: Section = Section {
    name: Some("status"),
    title: None,
    view: View::Detail {
        fields: &[
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
        ],
    },
};

static APPS: Section = Section {
    name: Some("apps"),
    title: Some("Apps"),
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
        ],
    },
};

pub async fn run(global_args: &GlobalArguments, args: Args) -> Result<(), TaskError> {
    let api_client = global_args.build_authed_api_client().await?;
    let mut report = Report::new(&args.output);
    let response =
        engines_api::get_engine_status(api_client.config(), crate::api::path_param(&args.slug)?)
            .await?;
    report.add(&STATUS, &response)?;
    let response = engines_api::list_all_applications(
        api_client.config(),
        Some(args.slug.as_str()),
        None,
        None,
    )
    .await?;
    report.add(&APPS, &response)?;
    report.print()
}
