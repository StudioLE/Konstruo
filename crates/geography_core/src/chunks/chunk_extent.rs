//! Extent of a chunk rectangle on the British National Grid.

use crate::BngCoordinates;

/// Extent of a chunk rectangle on the British National Grid.
///
/// - Reads as OSGB36 / British National Grid, EPSG:27700
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChunkExtent {
    /// South west corner.
    pub min: BngCoordinates,
    /// North east corner.
    pub max: BngCoordinates,
}
