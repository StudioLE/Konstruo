//! Request parameters for the height chunks subcommand.

use crate::prelude::*;

/// Parsed arguments for the height chunks subcommand.
#[derive(Args, Clone, Debug, Eq, PartialEq)]
pub struct HeightChunksRequest {
    /// Directory holding the source `GeoTIFF` files.
    #[arg(long, default_value = "../konstruo-gis/src/environment-agency")]
    pub source: PathBuf,
    /// Directory to write chunk files into.
    #[arg(long, default_value = "assets/terrain")]
    pub output: PathBuf,
}
