use crate::constraints::clamp_float::ClampFloat;
use crate::constraints::clamp_vec3::ClampVec3;
use crate::geometry::Orientation;
use crate::kinematics::Translation;
use crate::mathematics::constants::*;
use crate::mathematics::SphericalCoordinates;
use crate::CAMERA_MAX;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

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
                current: Vec3::new(DEFAULT_RADIUS, 0.0, 0.0),
                clamp: ClampVec3 {
                    x: ClampFloat::Fixed(10.0, CAMERA_MAX),
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
    /// Distance from the origin in metres.
    #[must_use]
    pub fn get_spherical_coordinates(&self) -> SphericalCoordinates {
        SphericalCoordinates::from(self.translation.current)
    }

    /// Get the orientation looking to the origin.
    #[must_use]
    pub fn get_orientation(&self) -> Quat {
        let spherical = self.get_spherical_coordinates();
        let yaw = spherical.get_azimuth() + HALF_PI;
        let pitch = spherical.get_polar();
        let roll = 0.0;
        Quat::from_euler(EulerRot::ZXY, yaw, pitch, roll)
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
        let direction = mouse.delta.normalize();
        let polar = direction.y * -1.0 * 0.1;
        let azimuthal = direction.x * -1.0 * 0.04;
        let displacement = Vec3::new(0.0, polar, azimuthal);
        self.translation
            .set_target_relative_to_current(displacement);
    }

    /// Orbit the camera to the specified orientation.
    pub(crate) fn orientate(&mut self, orientation: &[Orientation]) {
        let vector = Orientation::get_vector(orientation).normalize();
        let radius = self.get_spherical_coordinates().get_radius();
        let target = SphericalCoordinates::from_cartesian(vector)
            .vector
            .with_x(radius);
        self.translation.set_target(target);
    }

    /// Stop movement by removing the target.
    pub(super) fn stop(&mut self) {
        self.translation.remove_target();
    }
}
