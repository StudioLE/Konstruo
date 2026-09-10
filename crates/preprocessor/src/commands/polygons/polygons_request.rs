//! Request parameters for the polygons subcommand.

use crate::prelude::*;

/// Parsed arguments for the polygons subcommand.
#[derive(Args, Clone, Debug, Eq, PartialEq)]
pub struct PolygonsRequest {
    /// Directory holding the source `GeoPackage` files.
    #[arg(long, default_value = "../konstruo-gis/src")]
    pub source: PathBuf,
    /// Directory to write `GeoJSON` files into.
    #[arg(long, default_value = "assets/polygons")]
    pub output: PathBuf,
}
