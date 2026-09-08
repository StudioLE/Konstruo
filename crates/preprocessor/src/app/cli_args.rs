//! Parsed CLI arguments registered in the service container.

use crate::prelude::*;
use clap::Parser;

/// Parsed CLI arguments.
#[derive(Parser)]
#[command(
    name = "konstruo-preprocessor",
    about = "Generate Konstruo assets from source data"
)]
pub struct CliArgs {
    /// Set the log level.
    #[arg(long, value_enum)]
    pub log_level: Option<LogLevel>,
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Subcommand,
}

impl FromServices for CliArgs {
    type Error = Infallible;

    fn from_services(_: &ServiceProvider) -> Result<Self, Report<Self::Error>>
    where
        Self: Sized,
    {
        Ok(CliArgs::parse())
    }
}
