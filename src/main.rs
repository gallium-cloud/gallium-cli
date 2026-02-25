use crate::args::{Action, Invocation};
use crate::helpers::auth::get_login_response_for_saved_credentials;
use clap::Parser;

mod api;
mod args;
pub mod helpers;
mod task_common;
mod tasks;
mod tasks_internal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let invocation = Invocation::parse();

    match invocation.action {
        Some(Action::Proxy(args)) => return crate::tasks_internal::proxy::proxy(&args).await,
        Some(Action::Login) => return crate::tasks::login::login(&invocation.gargs).await.unwrap(),
        Some(Action::Logout) => return crate::tasks::login::logout(&invocation.gargs).await,
        Some(Action::Ssh(args)) => return crate::tasks::ssh::ssh(&invocation.gargs, &args).await,
        _ => (),
    };

    //TODO: on windows, double-clicking the EXE from the file browser will result in a console window that immediately closes
    match get_login_response_for_saved_credentials(&invocation.gargs).await {
        Ok(login_resp) => {
            if let Some(org) = login_resp.org {
                println!("Logged in to Gallium org: {}", org.name);
            }
            if let Some(avail_orgs) = login_resp.available_orgs {
                if !avail_orgs.is_empty() {
                    println!("{} orgs available.", avail_orgs.len());
                }
            }
        }
        Err(_) => {
            println!(
                "Oops, you're not logged-in. Try `{:?} login`",
                std::env::current_exe().unwrap()
            );
            return;
        }
    };
}
