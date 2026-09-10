use bevy::light::atmosphere::ScatteringMedium;
use bevy::light::Atmosphere;
use bevy::prelude::*;

/// Resolution of the falloff lookup table of the [`ScatteringMedium`].
const FALLOFF_RESOLUTION: u32 = 256;

/// Resolution of the phase lookup table of the [`ScatteringMedium`].
const PHASE_RESOLUTION: u32 = 256;

/// A graphical representation of the sky.
///
/// - Scatters [`Sun`](crate::Sun) into sky color, aerial perspective and ambient light
#[derive(Component)]
pub struct Sky;

/// A graphical representation of the sky.
pub struct SkyPlugin;

impl Sky {
    /// System to spawn [`Sky`] on startup.
    fn startup_system(mut commands: Commands, mut mediums: ResMut<Assets<ScatteringMedium>>) {
        let medium = mediums.add(ScatteringMedium::earth(
            FALLOFF_RESOLUTION,
            PHASE_RESOLUTION,
        ));
        commands.spawn(Sky::bundle(medium));
    }

    /// Create a bundle for [`Sky`].
    ///
    /// - Places the planet center below the origin on `Z`, overriding the `Y` that
    ///   [`Atmosphere`] assumes, this being a Z-up world
    fn bundle(medium: Handle<ScatteringMedium>) -> impl Bundle {
        let atmosphere = Atmosphere::earth(medium);
        let translation = Vec3::NEG_Z * atmosphere.inner_radius;
        (Sky, atmosphere, Transform::from_translation(translation))
    }
}

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Sky::startup_system);
    }
}
