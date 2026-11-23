// Generated from the Gallium OpenAPI specs. Do not edit.

use crate::args::GlobalArguments;
use crate::task_common::error::TaskError;

pub mod app_list;
pub mod engine_status;

#[derive(clap::Subcommand)]
pub enum GeneratedAction {
    /// Manage container engine applications
    #[command(subcommand)]
    App(AppCommand),
    /// Manage container engines
    #[command(subcommand)]
    Engine(EngineCommand),
}

#[derive(clap::Subcommand)]
pub enum AppCommand {
    /// List the apps in your Gallium org
    List(app_list::Args),
}

#[derive(clap::Subcommand)]
pub enum EngineCommand {
    /// Show the status of a container engine
    Status(engine_status::Args),
}

pub async fn run(gargs: &GlobalArguments, action: GeneratedAction) -> Result<(), TaskError> {
    match action {
        GeneratedAction::App(command) => match command {
            AppCommand::List(args) => app_list::run(gargs, args).await,
        },
        GeneratedAction::Engine(command) => match command {
            EngineCommand::Status(args) => engine_status::run(gargs, args).await,
        },
    }
}
