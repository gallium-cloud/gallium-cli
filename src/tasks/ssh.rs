use crate::api::vm_service_api::entities::GetWsUrlForVmServiceQueryParams;
use crate::api::vm_service_api::get_ws_url;

#[derive(clap::Parser)]
pub(crate) struct SshArguments {
    destination: String,

    #[arg(last = true)]
    additional_ssh_args: Vec<String>,
}

pub(crate) async fn ssh(gargs: &crate::args::GlobalArguments, args: &SshArguments) {
    let host = match args.destination.split_once('@') {
        Some((_, host)) => host,
        None => args.destination.as_str(),
    };

    let access_token = match crate::tasks::login::get_access_token(
        gargs.get_api_url(),
        &gargs.gallium_org,
    )
    .await
    {
        Ok(access_token) => access_token,
        Err(_) => {
            eprintln!(
                "Ooops, you're not logged-in. Try `{:?} login`",
                std::env::current_exe().unwrap()
            );
            return;
        }
    };

    let ws_url = match get_ws_url(
        gargs.get_api_url(),
        &access_token,
        &GetWsUrlForVmServiceQueryParams {
            host: host.to_string(),
            port: "22".into(),
        },
    )
    .await
    {
        Ok(ws_url) => ws_url,
        Err(e) => {
            eprintln!("Something went wrong: {:?}", e);
            return;
        }
    };

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
    .expect("ssh run");

    if let Some(code) = output.status.code() {
        std::process::exit(code);
    }
    std::process::exit(1);
}
