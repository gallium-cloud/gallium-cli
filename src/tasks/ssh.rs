use crate::api::vm_service_api::entities::GetWsUrlForVmServiceQueryParams;
use crate::helpers::auth::get_login_response_for_saved_credentials;

use crate::task_common::error::TaskError;
use snafu::prelude::*;

#[derive(clap::Parser)]
pub(crate) struct SshArguments {
    destination: String,

    #[arg(last = true)]
    additional_ssh_args: Vec<String>,
}

pub(crate) async fn ssh(
    gargs: &crate::args::GlobalArguments,
    args: &SshArguments,
) -> Result<(), TaskError> {
    let host = match args.destination.split_once('@') {
        Some((_, host)) => host,
        None => args.destination.as_str(),
    };

    let access_token = get_login_response_for_saved_credentials(gargs)
        .await?
        .access_token
        .ok_or_else(|| TaskError::ApiResponseMissingField {
            field: "accessToken",
        })?;

    let vm_service_api = gargs.build_api_client()?.vm_service_api();

    let ws_url = vm_service_api
        .get_ws_url_for_vm_service(
            &access_token,
            &GetWsUrlForVmServiceQueryParams {
                host: host.to_string(),
                port: "22".into(),
            },
        )
        .await?
        .url
        .ok_or_else(|| TaskError::ApiResponseMissingField { field: "ws:URL" })?;

    let output = duct::cmd(
        "ssh",
        vec![
            String::from("-o"),
            format!(
                "ProxyCommand={} proxy \"{}\"",
                std::env::current_exe().unwrap().display(),
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
