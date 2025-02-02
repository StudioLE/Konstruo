use crate::mathematics::{SphericalCoordinates, EIGHTH_PI, QUARTER_PI};
use crate::ENVIRONMENT_MAX;
use bevy::prelude::*;

/// A directional light source representing the sun.
#[derive(Component)]
#[require(DirectionalLight(create_light), Transform(create_transform))]
pub struct Sun;

/// A directional light source representing the sun.
pub struct SunPlugin;

impl Sun {
    /// System to spawn [`Sun`] on startup.
    pub fn startup_system(mut commands: Commands) {
        commands.spawn(Sun);
    }
}

impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Sun::startup_system);
    }
}

fn create_light() -> DirectionalLight {
    DirectionalLight {
        illuminance: 3_000.0,
        // illuminance: light_consts::lux::OVERCAST_DAY,
        // illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
        // illuminance: light_consts::lux::FULL_DAYLIGHT,
        shadows_enabled: true,
        ..default()
    }
}

fn create_transform() -> Transform {
    let translation =
        SphericalCoordinates::new(ENVIRONMENT_MAX - 1000.0, EIGHTH_PI, QUARTER_PI).to_cartesian();
    Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Z)
}
