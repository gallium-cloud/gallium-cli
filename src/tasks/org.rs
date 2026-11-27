use crate::args::GlobalArguments;
use crate::helpers::output::{Column, Format, OutputArgs, render_list};
use crate::task_common::error::TaskError;
use gallium_api_client::models::GalliumLoginResponse;

#[derive(clap::Subcommand)]
pub(crate) enum OrgCommand {
    /// List the orgs available to work with
    List(ListArguments),
}

#[derive(clap::Args)]
pub(crate) struct ListArguments {
    #[command(flatten)]
    output: OutputArgs,
}

#[derive(serde::Serialize, Debug, PartialEq, Eq)]
struct OrgList {
    orgs: Vec<OrgRow>,
}

#[derive(serde::Serialize, Debug, PartialEq, Eq)]
struct OrgRow {
    slug: String,
    name: String,
    parent: Option<String>,
    current: bool,
}

const COLUMNS: &[Column] = &[
    Column {
        header: "SLUG",
        pointer: "/slug",
        format: Format::Text,
    },
    Column {
        header: "NAME",
        pointer: "/name",
        format: Format::Text,
    },
    Column {
        header: "PARENT",
        pointer: "/parent",
        format: Format::Text,
    },
    Column {
        header: "CURRENT",
        pointer: "/current",
        format: Format::Boolean,
    },
];

pub(crate) async fn org(
    global_args: &GlobalArguments,
    command: OrgCommand,
) -> Result<(), TaskError> {
    match command {
        OrgCommand::List(args) => list(global_args, &args).await,
    }
}

async fn list(global_args: &GlobalArguments, args: &ListArguments) -> Result<(), TaskError> {
    let api_client = global_args.build_authed_api_client().await?;
    let response = api_client.refresh_session().await?;
    render_list(&org_list(&response), "/orgs", COLUMNS, &args.output)
}

fn org_list(response: &GalliumLoginResponse) -> OrgList {
    let current = response.org.as_ref().map(|org| org.slug.as_str());
    let orgs = response
        .available_orgs
        .iter()
        .flatten()
        .map(|org| OrgRow {
            slug: org.slug.clone(),
            name: org.name.clone(),
            parent: org.parent_org.as_ref().map(|parent| parent.slug.clone()),
            current: current == Some(org.slug.as_str()),
        })
        .collect();
    OrgList { orgs }
}
