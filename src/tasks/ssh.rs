use crate::helpers::env::current_exe;
use crate::task_common::error::TaskError;
use gallium_api_client::apis::virtual_machine_api;
use snafu::prelude::*;

#[derive(clap::Parser)]
pub(crate) struct SshArguments {
    destination: String,

    #[arg(last = true)]
    additional_ssh_args: Vec<String>,
}

pub(crate) async fn ssh(
    global_args: &crate::args::GlobalArguments,
    args: &SshArguments,
) -> Result<(), TaskError> {
    let host = match args.destination.split_once('@') {
        Some((_, host)) => host,
        None => args.destination.as_str(),
    };

    let api_client = global_args.build_authed_api_client().await?;

    let port = "22";
    let ws_url = virtual_machine_api::get_ws_url_for_vm_service(api_client.config(), port, host)
        .await?
        .url
        .ok_or_else(|| TaskError::ApiResponseMissingField { field: "ws:URL" })?;

    let output = duct::cmd(
        "ssh",
        vec![
            String::from("-o"),
            format!(
                "ProxyCommand={} proxy \"{}\"",
                current_exe()?.display(),
                ws_url
            ),
        ]
        .into_iter()
        .chain(args.additional_ssh_args.clone())
        .chain(std::iter::once(args.destination.clone())),
    )
    .unchecked()
    .run()
    .whatever_context::<_, TaskError>("ssh run")?;

    if let Some(code) = output.status.code() {
        std::process::exit(code);
    } else {
        std::process::exit(1);
    }
}
