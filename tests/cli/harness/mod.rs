pub mod api_mock;
pub mod browser_login;
mod cli_invocation;
mod cli_sandbox;

use std::time::Duration;

pub use cli_invocation::{CliInvocation, Outcome};
pub use cli_sandbox::CliSandbox;

pub const CONSOLE_URL: &str = "http://console.invalid";

const COMMAND_TIMEOUT: Duration = Duration::from_secs(30);
