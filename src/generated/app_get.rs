// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::helpers::engine::app_engine;
use crate::helpers::output::{Column, Field, Format, OutputArgs, Report, Section, View};
use crate::task_common::error::TaskError;
use gallium_api_client::apis::applications_api;

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

static SECTION: Section = Section {
    name: None,
    title: None,
    view: View::Detail {
        fields: &[
            Field {
                label: "Name",
                pointer: "/metadata/name",
                format: Format::Text,
            },
            Field {
                label: "ID",
                pointer: "/metadata/applicationId",
                format: Format::Text,
            },
            Field {
                label: "Phase",
                pointer: "/status/phase",
                format: Format::Text,
            },
            Field {
                label: "Created",
                pointer: "/metadata/createdAt",
                format: Format::Age,
            },
            Field {
                label: "Observed",
                pointer: "/status/observedAt",
                format: Format::Age,
            },
            Field {
                label: "Staged changes",
                pointer: "/status/stagedChanges/any",
                format: Format::Boolean,
            },
            Field {
                label: "Running lockfile",
                pointer: "/status/runningLockfile/id",
                format: Format::Text,
            },
        ],
    },
};

static CONTAINERS: Section = Section {
    name: None,
    title: Some("Containers"),
    view: View::List {
        items: "/status/containers",
        columns: &[
            Column {
                header: "NAME",
                pointer: "/name",
                format: Format::Text,
            },
            Column {
                header: "IMAGE",
                pointer: "/image",
                format: Format::Text,
            },
            Column {
                header: "STATE",
                pointer: "/current/state",
                format: Format::Text,
            },
            Column {
                header: "READY",
                pointer: "/current/ready",
                format: Format::Boolean,
            },
            Column {
                header: "RESTARTS",
                pointer: "/restarts",
                format: Format::Text,
            },
            Column {
                header: "STARTED",
                pointer: "/current/startedAt",
                format: Format::Age,
            },
        ],
    },
};

static CONDITIONS: Section = Section {
    name: None,
    title: Some("Conditions"),
    view: View::List {
        items: "/status/conditions",
        columns: &[
            Column {
                header: "TYPE",
                pointer: "/type",
                format: Format::Text,
            },
            Column {
                header: "STATUS",
                pointer: "/status",
                format: Format::Text,
            },
            Column {
                header: "REASON",
                pointer: "/reason",
                format: Format::Text,
            },
            Column {
                header: "MESSAGE",
                pointer: "/message",
                format: Format::Text,
            },
            Column {
                header: "SINCE",
                pointer: "/lastTransitionTime",
                format: Format::Age,
            },
        ],
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
    let response = applications_api::get_application(
        api_client.config(),
        crate::api::path_param(&engine)?,
        crate::api::path_param(&args.name)?,
    )
    .await?;
    report.add_views(&[&SECTION, &CONTAINERS, &CONDITIONS], &response)?;
    report.print()
}
