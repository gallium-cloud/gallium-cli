// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::task_common::error::TaskError;

pub mod app_get;
pub mod app_list;
pub mod app_new;
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
    /// Show a container engine app
    Get(app_get::Args),
    /// Create a new container engine app
    #[command(visible_alias = "create")]
    New(app_new::Args),
}

#[derive(clap::Subcommand)]
pub enum EngineCommand {
    /// Show the status of a container engine
    Status(engine_status::Args),
}

pub async fn run(global_args: &GlobalArguments, action: GeneratedAction) -> Result<(), TaskError> {
    match action {
        GeneratedAction::App(command) => match command {
            AppCommand::List(args) => app_list::run(global_args, args).await,
            AppCommand::Get(args) => app_get::run(global_args, args).await,
            AppCommand::New(args) => app_new::run(global_args, args).await,
        },
        GeneratedAction::Engine(command) => match command {
            EngineCommand::Status(args) => engine_status::run(global_args, args).await,
        },
    }
}
