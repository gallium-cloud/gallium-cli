mod proxy;

use clap::Parser;

#[derive(clap::Parser)]
struct Invocation {
    #[command(subcommand)]
    action: Option<Action>,
}

#[derive(clap::Subcommand)]
enum Action {
    #[clap(hide = true)]
    Proxy(crate::proxy::ProxyInvocation),
}

#[tokio::main]
async fn main() {
    let invocation = Invocation::parse();

    match invocation.action {
        Some(Action::Proxy(args)) => crate::proxy::proxy(args).await,
        _ => (),
    };
}
