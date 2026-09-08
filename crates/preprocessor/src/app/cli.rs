//! CLI bootstrapping and service initialization.

use crate::prelude::*;
use std::process::ExitCode;

/// Application entrypoint that bootstraps services and runs a subcommand.
pub struct Cli {
    /// Services resolved for the duration of the run.
    services: ServiceProvider,
}

impl Cli {
    /// Create a new [`Cli`] with the default service registrations.
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: ServiceBuilder::new()
                .with_app_services()
                .build()
                .expect_init(),
        }
    }

    /// Run the CLI to completion, returning the appropriate exit code.
    #[must_use]
    pub fn run(&self) -> ExitCode {
        if let Err(e) = self.run_subcommand() {
            error!("{}", e.render());
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }

    /// Dispatch the subcommand parsed from the command line.
    fn run_subcommand(&self) -> Result<(), StructuredError> {
        let handler = self.services.expect::<SubcommandHandler>();
        handler.run()
    }
}
