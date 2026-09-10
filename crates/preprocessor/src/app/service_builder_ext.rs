//! Extension trait for configuring a [`ServiceBuilder`] with application services.

use crate::prelude::*;

/// Register all application services on a [`ServiceBuilder`].
pub trait ServiceBuilderExt {
    /// Register all application services.
    fn with_app_services(self) -> Self;
}

impl ServiceBuilderExt for ServiceBuilder {
    fn with_app_services(self) -> Self {
        self.with_logging(create_logger)
            .with_type::<CellHeightChunkPaths>()
            .with_type::<CellHeightChunkReader>()
            .with_type::<CliArgs>()
            .with_type::<HeightChunksHandler>()
            .with_type::<PolygonReader>()
            .with_type::<PolygonWriter>()
            .with_type::<PolygonsHandler>()
            .with_type::<SubcommandHandler>()
    }
}

/// Build the [`Logger`] at the level the command line asks for.
#[cfg(not(test))]
fn create_logger(services: &ServiceProvider) -> Result<Logger, Report<ResolveError>> {
    let args = services.get::<CliArgs>()?;
    let logger = LoggerBuilder::new()
        .with_level(args.log_level.unwrap_or_default())
        .build();
    Ok(logger)
}

/// Build the [`Logger`] without parsing a command line the test harness owns.
#[cfg(test)]
#[expect(
    clippy::unnecessary_wraps,
    reason = "signature required by with_logging"
)]
fn create_logger(_services: &ServiceProvider) -> Result<Logger, Report<ResolveError>> {
    let logger = LoggerBuilder::new().with_level(LogLevel::Trace).build();
    Ok(logger)
}
