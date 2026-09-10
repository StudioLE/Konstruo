use bevy::prelude::*;
use konstruo_geography_core::HeightChunk;

/// Heights of one chunk, loaded as a Bevy asset.
///
/// Wraps [`HeightChunk`] because [`Asset`] requires [`TypePath`], which
/// `konstruo_geography_core` cannot derive without a `bevy` dependency.
#[derive(Asset, Clone, Debug, TypePath)]
pub struct HeightChunkAsset {
    /// The heights, at one spacing, for one chunk.
    pub chunk: HeightChunk,
}
