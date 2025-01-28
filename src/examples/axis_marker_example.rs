use crate::ui::AxisMarker;
use bevy::prelude::*;

pub struct AxisMarkerExample;

impl Plugin for AxisMarkerExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl AxisMarkerExample {
    fn startup_system(mut commands: Commands) {
        let bundle = (
            AxisMarker {
                thickness: 1.0,
                length: 10.0,
            },
            Transform::from_translation(Vec3::splat(10.0)),
        );
        commands.spawn(bundle);
    }
}
