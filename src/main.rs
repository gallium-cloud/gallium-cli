use clap::Parser;

mod login;
mod proxy;

#[derive(clap::Parser)]
struct Invocation {
    #[command(subcommand)]
    action: Option<Action>,

    #[arg(long, default_value = "https://api-staging.gallium.cloud/api")]
    api_root_url: String,
}

#[derive(clap::Subcommand)]
enum Action {
    #[clap(hide = true)]
    Proxy(crate::proxy::ProxyArguments),

    Login(crate::login::LoginArguments),

    Logout,
}

#[tokio::main]
async fn main() {
    let invocation = Invocation::parse();

    match invocation.action {
        Some(Action::Proxy(args)) => return crate::proxy::proxy(args).await,
        Some(Action::Login(args)) => return crate::login::login(args).await,
        Some(Action::Logout) => return crate::login::logout().await,
        _ => (),
    };

    let _access_token = match crate::login::get_access_token(&invocation.api_root_url).await {
        Ok(t) => t,
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
