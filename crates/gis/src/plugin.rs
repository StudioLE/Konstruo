use crate::{GeoJsonLoader, GeoJsonPolygons};
use bevy::prelude::*;

/// Registers the [`GeoJsonPolygons`] asset and its [`GeoJsonLoader`].
pub struct GeoJsonPlugin;

impl Plugin for GeoJsonPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GeoJsonPolygons>()
            .init_asset_loader::<GeoJsonLoader>();
    }
}
