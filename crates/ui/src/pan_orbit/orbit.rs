use crate::{PrimaryCamera, PRIMARY_CAMERA_ORDER};
#[cfg(not(target_arch = "wasm32"))]
use bevy::anti_alias::smaa::{Smaa, SmaaPreset};
use bevy::camera::Exposure;
use bevy::input::mouse::MouseMotion;
use bevy::light::AtmosphereEnvironmentMapLight;
use bevy::pbr::{AtmosphereMode, AtmosphereSettings};
#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::{ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel};
use bevy::prelude::*;
use konstruo_core::constants::{CAMERA_MAX, CAMERA_MIN, CAMERA_MSAA};
use konstruo_core::ClampVec3;
use konstruo_core::Translation;
use konstruo_core::{ClampFloat, HALF_PI, PI, TWO_PI};
use konstruo_geometry::Orientation;
use konstruo_geometry::Orientation::{Bottom, Top};
use konstruo_geometry::SphericalCoordinates;

pub(super) const DEFAULT_RADIUS: f32 = 250.0;
pub(super) const SPEED_MODIFIER: f32 = 2.0;

/// 3D orbital translation of the [`PanOrbitCameraPlugin`] around an origin.
///
/// The [`Orbit`] entity is a child of the [`Pan`] entity.
/// Therefore all translation is relative to the current pan translation.
///
/// Spherical coordinates are used unless explicitly stated otherwise.
#[derive(Component)]
pub struct Orbit {
    /// 3D oribtal translation.
    ///
    /// Spherical coordinates are used.
    pub(super) translation: Translation,
    /// Is dragging mode currently active?
    pub(super) dragging: bool,
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            translation: Translation {
                current: Vec3::new(DEFAULT_RADIUS, 0.0, -HALF_PI),
                clamp: ClampVec3 {
                    x: ClampFloat::Fixed(CAMERA_MIN, CAMERA_MAX),
                    y: ClampFloat::Fixed(0.0, PI),
                    z: ClampFloat::Wrapped(TWO_PI),
                },
                target: None,
                speed: Vec3::new(DEFAULT_RADIUS * SPEED_MODIFIER, HALF_PI, PI),
            },
            dragging: false,
        }
    }
}

impl Orbit {
    /// Create an [`Orbit`] with [`Camera`].
    ///
    /// - Takes ambient light and reflections from the atmosphere, so surfaces facing
    ///   away from the sun read as sky rather than black
    /// - If using `ContactShadows` a thickness of `0.6` works for window reveal; lower values are only visible with extreme zoom.
    pub(super) fn bundle() -> impl Bundle {
        let orbit = Orbit::default();
        let transform = orbit.get_cartesian_transform();
        (
            PrimaryCamera,
            orbit,
            transform,
            Camera3d::default(),
            Camera {
                order: PRIMARY_CAMERA_ORDER,
                ..default()
            },
            Exposure {
                // Default: `EV100_BLENDER, 9.7` (matches blender's lighting)
                // Overcast: `12.0`
                // Sunlight: `15.0`
                // TODO: Exposure needs proper calibration
                ev100: 13.5,
            },
            AtmosphereSettings {
                rendering_method: AtmosphereMode::Raymarched,
                ..default()
            },
            AtmosphereEnvironmentMapLight {
                // Default: `512`
                // MUST be a power of two.
                size: UVec2::splat(1024),
                ..default()
            },
            CAMERA_MSAA,
            #[cfg(not(target_arch = "wasm32"))]
            ScreenSpaceAmbientOcclusion {
                quality_level: ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
                ..default()
            },
            #[cfg(not(target_arch = "wasm32"))]
            Smaa {
                preset: SmaaPreset::Ultra,
            },
        )
    }
    /// Distance from the origin in metres.
    #[must_use]
    pub fn get_spherical_coordinates(&self) -> SphericalCoordinates {
        SphericalCoordinates::from(self.translation.current)
    }

    /// Get the orientation looking to the origin.
    #[must_use]
    pub fn get_orientation(&self) -> Quat {
        let spherical = self.get_spherical_coordinates();
        let polar = spherical.get_polar();
        let azimuth = spherical.get_azimuth() + HALF_PI;
        Quat::from_rotation_z(azimuth) * Quat::from_rotation_x(polar)
    }

    /// Get the cartesian translation from the origin.
    #[must_use]
    pub fn get_cartesian_translation(&self) -> Vec3 {
        self.get_spherical_coordinates().to_cartesian()
    }

    /// Get the cartesian translation and orientation looking to the origin.
    #[must_use]
    pub fn get_cartesian_transform(&self) -> Transform {
        Transform::from_translation(self.get_cartesian_translation())
            .with_rotation(self.get_orientation())
    }

    /// Orbit the camera in direction relative to the Azimuth.
    pub(super) fn in_direction(&mut self, direction: Vec3) {
        let distance = self.translation.speed * 0.1;
        self.translation
            .set_target_relative_to_current(direction * distance);
    }

    /// Orbit the camera in the direction of the mouse motion.
    pub(super) fn in_direction_of_motion(&mut self, mouse: MouseMotion) {
        if mouse.delta == Vec2::ZERO {
            return;
        }
        let direction = mouse.delta.normalize();
        let polar = direction.y * -1.0 * 0.1;
        let azimuthal = direction.x * -1.0 * 0.04;
        let displacement = Vec3::new(0.0, polar, azimuthal);
        self.translation
            .set_target_relative_to_current(displacement);
    }

    /// Orbit the camera to the specified orientation.
    pub(crate) fn orientate(&mut self, orientation: &[Orientation]) {
        let radius = self.get_spherical_coordinates().get_radius();
        let target = if orientation == [Top] {
            Vec3::new(radius, 0.0, -HALF_PI)
        } else if orientation == [Bottom] {
            Vec3::new(radius, PI, -HALF_PI)
        } else {
            let vector = Orientation::get_facing_in(orientation).normalize();
            SphericalCoordinates::from_cartesian(vector)
                .vector
                .with_x(radius)
        };
        self.translation.set_target(target);
    }

    /// Stop movement by removing the target.
    pub(super) fn stop(&mut self) {
        self.translation.remove_target();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use konstruo_geometry::Orientation::{Bottom, Front, Left, Top};

    #[test]
    #[allow(clippy::float_cmp)]
    fn orientate_top() {
        // Arrange
        let mut orbit = Orbit::default();

        // Act
        orbit.orientate(&[Top]);

        // Assert
        let target = orbit.translation.target.expect("target should be set");
        assert_eq!(target.x, DEFAULT_RADIUS);
        assert_eq!(target.y, 0.0);
        // -HALF_PI wrapped to [0, TWO_PI) equals 3 * HALF_PI
        assert_eq!(target.z, 3.0 * HALF_PI);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn orientate_bottom() {
        // Arrange
        let mut orbit = Orbit::default();

        // Act
        orbit.orientate(&[Bottom]);

        // Assert
        let target = orbit.translation.target.expect("target should be set");
        assert_eq!(target.x, DEFAULT_RADIUS);
        assert_eq!(target.y, PI);
        assert_eq!(target.z, 3.0 * HALF_PI);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn orientate_front() {
        // Arrange
        let mut orbit = Orbit::default();

        // Act
        orbit.orientate(&[Front]);

        // Assert
        let target = orbit.translation.target.expect("target should be set");
        assert_eq!(target.x, DEFAULT_RADIUS);
        assert_eq!(target.y, HALF_PI);
        // atan2(-1, 0) = -PI/2, wrapped to [0, TWO_PI) = 3*PI/2
        assert_eq!(target.z, 3.0 * HALF_PI);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn orientate_left() {
        // Arrange
        let mut orbit = Orbit::default();

        // Act
        orbit.orientate(&[Left]);

        // Assert
        let target = orbit.translation.target.expect("target should be set");
        assert_eq!(target.x, DEFAULT_RADIUS);
        assert_eq!(target.y, HALF_PI);
        assert_eq!(target.z, PI);
    }

    #[test]
    fn stop_removes_target() {
        // Arrange
        let mut orbit = Orbit::default();
        orbit.orientate(&[Top]);
        assert!(orbit.translation.target.is_some());

        // Act
        orbit.stop();

        // Assert
        assert!(orbit.translation.target.is_none());
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn get_spherical_coordinates_default() {
        // Arrange
        let orbit = Orbit::default();

        // Act
        let spherical = orbit.get_spherical_coordinates();

        // Assert
        assert_eq!(spherical.get_radius(), DEFAULT_RADIUS);
        assert_eq!(spherical.get_polar(), 0.0);
        assert_eq!(spherical.get_azimuth(), -HALF_PI);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn get_cartesian_translation_default() {
        // Arrange
        let orbit = Orbit::default();

        // Act
        let translation = orbit.get_cartesian_translation();

        // Assert
        assert_eq!(translation, Vec3::new(0.0, 0.0, DEFAULT_RADIUS));
    }
}
