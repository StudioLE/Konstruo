use bevy::prelude::*;
use std::f32::consts::TAU;

#[derive(Component)]
#[require(DirectionalLight(create_light), Transform(create_transform))]
pub struct Sun;

fn create_light() -> DirectionalLight {
    DirectionalLight {
        // illuminance: light_consts::lux::OVERCAST_DAY,
        illuminance: 5000.0,
        shadows_enabled: true,
        ..default()
    }
}

fn create_transform() -> Transform {
    Transform {
        translation: Vec3::new(0.0, 1000.0, 0.0),
        rotation: Quat::from_rotation_x(TAU / 8.0 * -1.0),
        ..default()
    }
}

pub fn spawn_sun(mut commands: Commands) {
    commands.spawn(Sun);
}
