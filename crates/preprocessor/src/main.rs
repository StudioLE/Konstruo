//! Binary entrypoint for `konstruo-preprocessor`.

use konstruo_preprocessor::prelude::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    Cli::new().run()
}
