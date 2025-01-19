use bevy::prelude::*;
use std::f32::consts::TAU;

#[derive(Component)]
#[require(DirectionalLight, Transform)]
pub struct Sun;

pub fn spawn_sun(mut commands: Commands) {
    let bundle = (
        Sun,
        DirectionalLight {
            // illuminance: light_consts::lux::OVERCAST_DAY,
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 1000.0, 0.0),
            rotation: Quat::from_rotation_x(TAU / 8.0 * -1.0),
            ..default()
        }
    );
    commands.spawn(bundle);
}
