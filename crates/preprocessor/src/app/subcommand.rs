//! Subcommand variants and dispatch handler.

use crate::prelude::*;
use clap::Subcommand as ClapSubcommand;

/// Available CLI subcommands.
#[derive(Clone, ClapSubcommand)]
pub enum Subcommand {
    /// Resample source height data into level-of-detail chunk files.
    HeightChunks(HeightChunksRequest),
}

/// Dispatch the selected [`Subcommand`] to its handler.
#[derive(FromServices)]
pub struct SubcommandHandler {
    /// Parsed CLI arguments.
    cli: Arc<CliArgs>,
    /// Handler of the `height-chunks` subcommand.
    height_chunks: Arc<HeightChunksHandler>,
}

impl SubcommandHandler {
    /// Execute the selected subcommand.
    pub fn run(&self) -> Result<(), StructuredError> {
        let command = self.cli.command.clone();
        match command {
            Subcommand::HeightChunks(request) => Ok(self.height_chunks.execute(request)?),
        }
    }
}
