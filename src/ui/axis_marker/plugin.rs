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

pub struct OriginMarkerPlugin;

impl Plugin for OriginMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl OriginMarkerPlugin {
    fn startup_system(mut commands: Commands) {
        let bundle = (
            AxisMarker {
                thickness: 0.1,
                length: 200.0,
            },
            Transform::from_translation(Vec3::ZERO),
        );
        commands.spawn(bundle);
    }
}
