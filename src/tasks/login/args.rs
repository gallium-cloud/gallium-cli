use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum LoginMethod {
    /// Provide username, password and OTP to the CLI directly
    Password,
}

#[derive(clap::Parser)]
pub struct LoginArguments {
    /// How the CLI should log you in
    #[arg(long, default_value = "password")]
    pub method: LoginMethod,
}
