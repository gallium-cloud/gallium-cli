use clap::Parser;

mod api;
mod login;
mod proxy;
mod ssh;

#[derive(clap::Parser)]
struct Invocation {
    #[command(flatten)]
    gargs: GlobalArguments,

    #[command(subcommand)]
    action: Option<Action>,
}

#[derive(clap::Args)]
struct GlobalArguments {
    #[arg(long, default_value = "https://api.gallium.cloud/api")]
    api_root_url: String,

    #[arg(short, long, default_missing_value= Option::None)]
    gallium_org: Option<String>,
}

#[derive(clap::Subcommand)]
enum Action {
    #[clap(hide = true)]
    Proxy(crate::proxy::ProxyArguments),

    Login,
    Logout,

    Ssh(crate::ssh::SshArguments),
}

#[tokio::main]
async fn main() {
    let invocation = Invocation::parse();

    match invocation.action {
        Some(Action::Proxy(args)) => return crate::proxy::proxy(&args).await,
        Some(Action::Login) => return crate::login::login(&invocation.gargs).await,
        Some(Action::Logout) => return crate::login::logout(&invocation.gargs).await,
        Some(Action::Ssh(args)) => return crate::ssh::ssh(&invocation.gargs, &args).await,
        _ => (),
    };

    let _access_token = match crate::login::get_access_token(
        &invocation.gargs.api_root_url,
        &invocation.gargs.gallium_org,
    )
    .await
    {
        Ok(access_token) => access_token,
        Err(_) => {
            println!(
                "Ooops, you're not logged-in. Try `{:?} login`",
                std::env::current_exe().unwrap()
            );
            return;
        }
    };

    println!("you're logged in... but I don't do anything, yet.");
}
