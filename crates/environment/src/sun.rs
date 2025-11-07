use bevy::light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use konstruo_core::constants::{CAMERA_MAX, ENVIRONMENT_MAX};
use konstruo_core::{ONE_DEGREE_IN_RADIANS, PI};
use konstruo_geometry::SphericalCoordinates;

/// A directional light source representing the sun.
#[derive(Component)]
pub struct Sun;

/// A directional light source representing the sun.
pub struct SunPlugin;

impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 1024 * 2 })
            // .insert_resource(AmbientLight {
            //     color: tailwind::AMBER_50.into(),
            //     brightness: 1_000.0,
            // })
            .add_systems(Startup, Sun::startup_system);
    }
}

impl Sun {
    /// System to spawn [`Sun`] on startup.
    ///
    /// A secondary directional light is included so openings have
    /// depth even when shaded from the sun.
    pub fn startup_system(mut commands: Commands) {
        let azimuth = -157.5;
        commands.spawn(Sun::bundle(ONE_DEGREE_IN_RADIANS * azimuth, 3_000.0));
        commands.spawn(Sun::bundle(
            ONE_DEGREE_IN_RADIANS * (azimuth + 180.0),
            100.0,
        ));
    }

    fn bundle(azimuth: f32, illuminance: f32) -> impl Bundle {
        let translation =
            SphericalCoordinates::new(ENVIRONMENT_MAX - 1000.0, PI / 7.0, azimuth).to_cartesian();
        (
            Sun,
            DirectionalLight {
                illuminance,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Z),
            CascadeShadowConfigBuilder {
                maximum_distance: CAMERA_MAX,
                ..default()
            }
            .build(),
        )
    }
}
