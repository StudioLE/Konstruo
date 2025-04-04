use crate::ui::AxisMarkerFactory;
use bevy::prelude::*;

pub struct AxisMarkerExample;

impl Plugin for AxisMarkerExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl AxisMarkerExample {
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
        factory.spawn(Some(Vec3::splat(10.0)), Some(1.0), Some(10.0));
    }
}
