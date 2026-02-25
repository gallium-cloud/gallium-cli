use crate::args::{Action, Invocation};
use clap::Parser;

mod api;
mod args;
pub mod helpers;
mod tasks;
mod tasks_internal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let invocation = Invocation::parse();

    match invocation.action {
        Some(Action::Proxy(args)) => return crate::tasks_internal::proxy::proxy(&args).await,
        Some(Action::Login) => return crate::tasks::login::login(&invocation.gargs).await,
        Some(Action::Logout) => return crate::tasks::login::logout(&invocation.gargs).await,
        Some(Action::Ssh(args)) => return crate::tasks::ssh::ssh(&invocation.gargs, &args).await,
        _ => (),
    };

    let _access_token = match crate::tasks::login::get_access_token(
        &invocation.gargs.api_root_url,
        &invocation.gargs.gallium_org,
    )
    .await
    {
        Ok(access_token) => access_token,
        Err(_) => {
            println!(
                "Oops, you're not logged-in. Try `{:?} login`",
                std::env::current_exe().unwrap()
            );
            return;
        }
    };
}
