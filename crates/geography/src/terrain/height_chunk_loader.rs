use crate::HeightChunkAsset;
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use konstruo_geography_core::{ChunkIndex, HeightChunk, HeightChunkError, HeightSpacing};
use serde::{Deserialize, Serialize};
use std::io::Error as IoError;
use studiole_report::prelude::*;
use thiserror::Error;

/// Settings of a [`HeightChunkLoader`] load.
///
/// - Both fields default to [`None`], so a caller that forgets to set them
///   gets an error rather than a chunk decoded against the wrong spacing
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct HeightChunkSettings {
    /// Interval between vertices.
    ///
    /// Default: `None`
    pub spacing: Option<HeightSpacing>,
    /// Position of the chunk.
    ///
    /// Default: `None`
    pub index: Option<ChunkIndex>,
}

/// An [`AssetLoader`] for the height chunks written by `konstruo-preprocessor`.
///
/// The files hold little-endian `f32` with no header, so the spacing and index
/// come from [`HeightChunkSettings`] rather than the bytes.
#[derive(Default, TypePath)]
pub struct HeightChunkLoader;

impl AssetLoader for HeightChunkLoader {
    type Asset = HeightChunkAsset;
    type Settings = HeightChunkSettings;
    type Error = HeightChunkLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = to_asset(settings, &bytes)?;
        debug!(
            "Loaded height chunk {} from {}",
            asset.chunk.index,
            load_context.path()
        );
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["f32"]
    }
}

/// Create a [`HeightChunkAsset`] from chunk bytes and the settings of the load.
///
/// # Errors
///
/// - [`HeightChunkLoaderError::MissingSpacing`] IF the settings hold no spacing
/// - [`HeightChunkLoaderError::MissingIndex`] IF the settings hold no index
/// - [`HeightChunkLoaderError::Decode`] IF the byte count does not match the spacing
fn to_asset(
    settings: &HeightChunkSettings,
    bytes: &[u8],
) -> Result<HeightChunkAsset, HeightChunkLoaderError> {
    let spacing = settings
        .spacing
        .ok_or(HeightChunkLoaderError::MissingSpacing)?;
    let index = settings.index.ok_or(HeightChunkLoaderError::MissingIndex)?;
    let chunk = HeightChunk::from_bytes(spacing, index, bytes)?;
    Ok(HeightChunkAsset { chunk })
}

/// Errors returned by [`HeightChunkLoader`].
#[derive(Debug, Error)]
pub enum HeightChunkLoaderError {
    /// Failed to read the asset bytes.
    #[error("failed to read height chunk asset")]
    Read(#[from] IoError),
    /// The settings hold no spacing to decode against.
    #[error("height chunk settings hold no spacing")]
    MissingSpacing,
    /// The settings hold no index to place the chunk.
    #[error("height chunk settings hold no index")]
    MissingIndex,
    /// Failed to decode the asset bytes.
    #[error("failed to decode height chunk asset")]
    Decode(#[from] Report<HeightChunkError>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use konstruo_geography_core::HeightChunk;

    /// Spacing with the fewest vertices, so tests stay small.
    const SPACING: HeightSpacing = HeightSpacing::Sixteen;

    #[test]
    fn height_chunk_loader_to_asset() {
        // Arrange
        let chunk = HeightChunk::mock(SPACING);
        let settings = HeightChunkSettings {
            spacing: Some(SPACING),
            index: Some(chunk.index),
        };

        // Act
        let output = to_asset(&settings, &chunk.to_bytes());

        // Assert
        assert_eq!(output.expect("should decode").chunk, chunk);
    }

    /// Settings left at their default are an error, not a guess at the spacing.
    #[test]
    fn height_chunk_loader_to_asset_default_settings() {
        // Arrange
        let chunk = HeightChunk::mock(SPACING);
        let settings = HeightChunkSettings::default();

        // Act
        let output = to_asset(&settings, &chunk.to_bytes());

        // Assert
        assert!(matches!(
            output,
            Err(HeightChunkLoaderError::MissingSpacing)
        ));
    }

    #[test]
    fn height_chunk_loader_to_asset_missing_index() {
        // Arrange
        let chunk = HeightChunk::mock(SPACING);
        let settings = HeightChunkSettings {
            spacing: Some(SPACING),
            index: None,
        };

        // Act
        let output = to_asset(&settings, &chunk.to_bytes());

        // Assert
        assert!(matches!(output, Err(HeightChunkLoaderError::MissingIndex)));
    }

    #[test]
    fn height_chunk_loader_to_asset_short_bytes() {
        // Arrange
        let chunk = HeightChunk::mock(SPACING);
        let mut bytes = chunk.to_bytes();
        bytes.pop();
        let settings = HeightChunkSettings {
            spacing: Some(SPACING),
            index: Some(chunk.index),
        };

        // Act
        let output = to_asset(&settings, &bytes);

        // Assert
        assert!(matches!(output, Err(HeightChunkLoaderError::Decode(_))));
    }
}
