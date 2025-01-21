use beach_core::constraints::clamp_float::ClampFloat;
use beach_core::constraints::clamp_vec3::ClampVec3;
use beach_core::geometry::Orientation;
use beach_core::kinematics::Translation;
use beach_core::mathematics::constants::*;
use beach_core::mathematics::spherical_coordinate_system::*;
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
                    x: ClampFloat::Fixed(10.0, 2500.0),
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
    pub fn get_radius(&self) -> f32 {
        self.translation.current.x
    }

    /// Angle from the zenith in radians.
    ///
    /// Also known as the polar angle or zenith angle
    /// - <https://mathworld.wolfram.com/PolarAngle.html>
    /// - <https://mathworld.wolfram.com/ZenithAngle.html>
    ///
    /// Zenith is the positive Z axis or north pole.
    pub fn get_polar(&self) -> f32 {
        self.translation.current.y
    }

    /// Angle from the horizon in radians.
    /// - <https://mathworld.wolfram.com/Colatitude.html>
    pub fn get_altitude(&self) -> f32 {
        HALF_PI - self.get_polar()
    }

    /// Angle from the X axis in the XY plane.
    /// -<https://mathworld.wolfram.com/Azimuth.html>
    pub fn get_azimuth(&self) -> f32 {
        self.translation.current.z
    }

    /// Get the orientation looking to the origin.
    pub fn get_orientation(&self) -> Quat {
        let z = self.get_azimuth() + HALF_PI;
        let x = self.get_polar();
        Quat::from_euler(EulerRot::ZXY, z, x, 0.0)
    }

    /// Get the cartesian translation from the origin.
    pub fn get_cartesian_translation(&self) -> Vec3 {
        spherical_to_cartesian(self.get_radius(), self.get_polar(), self.get_azimuth())
    }

    /// Get the cartesian translation and orientation looking to the origin.
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
        let spherical = cartesian_to_spherical(vector).with_x(self.get_radius());
        self.translation.set_target(spherical);
    }

    /// Stop movement by removing the target.
    pub(super) fn stop(&mut self) {
        self.translation.remove_target();
    }
}
