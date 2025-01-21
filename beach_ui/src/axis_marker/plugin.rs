use bevy::prelude::*;

use super::*;

/// Plugin to enable the creation of [`AxisMarker`].
pub struct AxisMarkerPlugin;

impl Plugin for AxisMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, AxisMarkerMaterials::startup_system)
            .add_systems(Startup, AxisMarkerMeshes::startup_system)
            .add_systems(Update, AxisMarker::added_system);
    }
}
