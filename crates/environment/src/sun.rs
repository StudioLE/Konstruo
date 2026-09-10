use bevy::light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::light_consts::lux::RAW_SUNLIGHT;
use bevy::prelude::*;
use konstruo_core::constants::ENVIRONMENT_MAX;
use konstruo_core::ONE_DEGREE_IN_RADIANS;
use konstruo_geometry::SphericalCoordinates;

/// Altitude of the sun above the horizon in degrees.
///
/// - Grazes the terrain enough that neighboring facets take visibly different shading
/// - Matches the altitude a hillshade is conventionally rendered at
const ALTITUDE: f32 = 45.0;

/// Azimuth of the sun in degrees, counterclockwise from east.
///
/// - Places the sun in the north west
/// - Relief lit from the south reads inverted, ridges appearing as valleys
const AZIMUTH: f32 = 135.0;

/// A directional light source representing the sun.
#[derive(Component)]
pub struct Sun;

/// A directional light source representing the sun.
pub struct SunPlugin;

impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap {
            // Default: `2048`
            // MUST be a power of two, or cascade positioning goes unstable.
            // Every cascade allocates one, so the cost scales with `num_cascades`.
            size: 4096,
        })
        .add_systems(Startup, Sun::startup_system);
    }
}

impl Sun {
    /// System to spawn [`Sun`] on startup.
    ///
    /// - Spawns one light, the sky filling the faces it misses
    fn startup_system(mut commands: Commands) {
        commands.spawn(Sun::bundle());
    }

    /// Create a bundle for [`Sun`].
    fn bundle() -> impl Bundle {
        let polar = ONE_DEGREE_IN_RADIANS * (90.0 - ALTITUDE);
        let azimuth = ONE_DEGREE_IN_RADIANS * AZIMUTH;
        let translation =
            SphericalCoordinates::new(ENVIRONMENT_MAX - 1000.0, polar, azimuth).to_cartesian();
        (
            Sun,
            DirectionalLight {
                // Default: `AMBIENT_DAYLIGHT`
                // `RAW_SUNLIGHT` is used as our camera applies an `Atmosphere` to scatter the light.
                // All other consts are proxies for this.
                illuminance: RAW_SUNLIGHT,
                shadow_maps_enabled: true,
                ..default()
            },
            Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Z),
            CascadeShadowConfigBuilder {
                // Default: `0.1`
                // `10` appears to work on buildings, rarely would the camera be closer than this.
                minimum_distance: 10.0,
                // Default: `150`
                // `750` is about the point building shadows become too small to see.
                maximum_distance: 750.0,
                // Default: `10`
                // `50` keeps the first cascade small enough to resolve window reveals,
                // which is what contact shadows previously stood in for.
                first_cascade_far_bound: 50.0,
                // Default: `0.2`
                // `0.4` Has more blending reducing the jump between cascades
                overlap_proportion: 0.4,
                ..default()
            }
            .build(),
        )
    }
}
