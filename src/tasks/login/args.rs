use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum LoginMethod {
    /// Confirm the login in the Gallium console through a web browser
    Browser,

    /// Provide username, password and OTP to the CLI directly
    Password,
}

#[derive(clap::Parser)]
pub struct LoginArguments {
    /// How the CLI should log you in
    #[arg(long, default_value = "browser")]
    pub method: LoginMethod,

    /// Don't open the browser automatically
    #[arg(long)]
    pub no_auto_open: bool,
}
