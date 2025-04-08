use bevy::prelude::*;

use super::*;

/// Plugin to create an [`AxisMarker`] at the origin.
pub struct OriginMarkerPlugin;

impl Plugin for OriginMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl OriginMarkerPlugin {
    fn startup_system(
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let factory = AxisMarkerFactory {
            commands,
            meshes,
            materials,
        };
        factory.spawn(None, Some(0.1), Some(200.0));
    }
}
