// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::task_common::error::TaskError;

pub mod app_delete;
pub mod app_deploy;
pub mod app_discard;
pub mod app_get;
pub mod app_list;
pub mod app_logs;
pub mod app_new;
pub mod app_restart;
pub mod app_start;
pub mod app_stop;
pub mod engine_list;
pub mod engine_status;

#[derive(clap::Subcommand)]
pub enum GeneratedAction {
    /// Manage container engine applications
    #[command(subcommand, visible_alias = "apps")]
    App(AppCommand),
    /// Manage container engines
    #[command(subcommand, visible_alias = "engines")]
    Engine(EngineCommand),
}

#[derive(clap::Subcommand)]
pub enum AppCommand {
    /// List the apps in your Gallium org
    List(app_list::Args),
    /// Delete an app
    #[command(visible_alias = "rm")]
    Delete(app_delete::Args),
    /// Deploy an app
    ///
    /// Deploy any staged changes to the running container engine app.
    /// The application may become unavailable during deployment.
    Deploy(app_deploy::Args),
    /// Discard any staged changes
    ///
    /// Resets the app's working definition to match what's currently running
    Discard(app_discard::Args),
    /// Show a container engine app
    Get(app_get::Args),
    /// Print the logs for an app's container
    Logs(app_logs::Args),
    /// Create a new container engine app
    #[command(visible_alias = "create")]
    New(app_new::Args),
    /// Completely stop a running app
    Stop(app_stop::Args),
    /// Start up a previously stopped app
    Start(app_start::Args),
    /// Restart a container
    Restart(app_restart::Args),
}

#[derive(clap::Subcommand)]
pub enum EngineCommand {
    /// List the container engines in your Gallium org
    List(engine_list::Args),
    /// Show the status of a container engine
    Status(engine_status::Args),
}

pub async fn run(global_args: &GlobalArguments, action: GeneratedAction) -> Result<(), TaskError> {
    match action {
        GeneratedAction::App(command) => match command {
            AppCommand::List(args) => app_list::run(global_args, args).await,
            AppCommand::Delete(args) => app_delete::run(global_args, args).await,
            AppCommand::Deploy(args) => app_deploy::run(global_args, args).await,
            AppCommand::Discard(args) => app_discard::run(global_args, args).await,
            AppCommand::Get(args) => app_get::run(global_args, args).await,
            AppCommand::Logs(args) => app_logs::run(global_args, args).await,
            AppCommand::New(args) => app_new::run(global_args, args).await,
            AppCommand::Stop(args) => app_stop::run(global_args, args).await,
            AppCommand::Start(args) => app_start::run(global_args, args).await,
            AppCommand::Restart(args) => app_restart::run(global_args, args).await,
        },
        GeneratedAction::Engine(command) => match command {
            EngineCommand::List(args) => engine_list::run(global_args, args).await,
            EngineCommand::Status(args) => engine_status::run(global_args, args).await,
        },
    }
}
