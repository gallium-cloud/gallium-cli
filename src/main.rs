use crate::args::{Action, Invocation};
use crate::task_common::error::TaskError;
use clap::Parser;

mod api;
mod args;
mod generated;
pub mod helpers;
mod task_common;
mod tasks;
mod tasks_internal;

#[tokio::main(flavor = "current_thread")]
#[snafu::report]
async fn main() -> Result<(), TaskError> {
    let invocation = Invocation::parse();

    match invocation.action {
        Some(Action::DlQemuImg) => crate::tasks_internal::qemu_img::dl_qemu_img().await,
        Some(Action::Proxy(args)) => crate::tasks_internal::proxy::proxy(&args).await,
        Some(Action::Generated(action)) => {
            crate::generated::run(&invocation.global_args, action).await
        }
        Some(Action::Export(args)) => {
            crate::tasks::export::export_main(&invocation.global_args, args).await
        }
        Some(Action::Import(args)) => {
            crate::tasks::import::import_main(&invocation.global_args, args).await
        }
        Some(Action::Login(args)) => {
            crate::tasks::login::login(&invocation.global_args, args).await
        }
        Some(Action::Logout) => crate::tasks::logout::logout(&invocation.global_args).await,
        Some(Action::Org(command)) => {
            crate::tasks::org::org(&invocation.global_args, command).await
        }
        Some(Action::Ssh(args)) => crate::tasks::ssh::ssh(&invocation.global_args, &args).await,
        Some(Action::Version) => crate::tasks::version::print_version_info(),
        None => {
            //TODO: on windows, double-clicking the EXE from the file browser will result in a console window that immediately closes
            let api_client = match invocation.global_args.build_authed_api_client().await {
                Ok(api_client) => api_client,
                Err(TaskError::NotLoggedIn) => {
                    println!(
                        "Oops, you're not logged-in. Try `{} login`",
                        crate::helpers::cli::exe_display_name()
                    );
                    return Ok(());
                }
                Err(e) => return Err(e),
            };
            let login_resp = api_client.refresh_session().await?;
            if let Some(org) = login_resp.org {
                println!("Logged in to Gallium org: {}", org.name);
            }
            if let Some(avail_orgs) = login_resp.available_orgs
                && !avail_orgs.is_empty()
            {
                println!("{} orgs available.", avail_orgs.len());
            }
            Ok(())
        }
    }
}
