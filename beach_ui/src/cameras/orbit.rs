use beach_core::constraints::clamp_float::ClampFloat;
use beach_core::constraints::clamp_vec3::ClampVec3;
use beach_core::mathematics::constants::*;
use beach_core::mathematics::spherical_coordinate_system::*;
use beach_core::movement::direct::DirectMovement;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

/// Orbital state of the camera.
#[derive(Component)]
pub struct Orbit {
    pub movement: DirectMovement,
    pub dragging: bool,
}

impl Default for Orbit {
    fn default() -> Self {
        Self {
            movement: DirectMovement {
                current: Vec3::new(250.0, 0.0, 0.0),
                clamp: ClampVec3 {
                    x: ClampFloat::Fixed(10.0, 2500.0),
                    y: ClampFloat::Fixed(0.0, PI),
                    z: ClampFloat::Wrapped(TWO_PI),
                },
                target: None,
                speed: Vec3::new(250.0, HALF_PI, PI),
            },
            dragging: false,
        }
    }
}

impl Orbit {
    /// Distance from the origin in metres.
    pub fn get_radius(&self) -> f32 {
        self.movement.current.x
    }

    /// Angle from the zenith in radians.
    pub fn get_polar(&self) -> f32 {
        self.movement.current.y
    }

    /// Angle from the zenith in radians.
    pub fn get_altitude(&self) -> f32 {
        HALF_PI - self.get_polar()
    }

    /// Angle from Z to the entity in the XZ plane.
    pub fn get_azimuth(&self) -> f32 {
        self.movement.current.z
    }

    /// Get the position and rotation of the camera.
    pub fn get_transform(&self) -> Transform {
        Transform::from_translation(self.get_position()).with_rotation(self.get_rotation())
    }

    /// Get the position relative to the origin.
    pub fn get_position(&self) -> Vec3 {
        spherical_to_cartesian(self.get_radius(), self.get_polar(), self.get_azimuth())
    }

    /// Get the rotation looking to the origin.
    pub fn get_rotation(&self) -> Quat {
        let z = self.get_azimuth() + HALF_PI;
        let x = self.get_polar();
        Quat::from_euler(EulerRot::ZXY, z, x, 0.0)
    }

    /// Orbit the camera in direction relative to the Azimuth.
    pub(crate) fn in_direction(&mut self, direction: Vec3, modifier: f32) {
        let velocity = direction * self.movement.speed * modifier;
        self.movement.set_target_relative_to_position(velocity);
    }

    /// Orbit the camera in the direction of the mouse motion.
    pub(crate) fn in_direction_of_motion(&mut self, mouse: MouseMotion) {
        let direction = mouse.delta.normalize();
        let polar = direction.y * -1.0 * 0.1;
        let azimuthal = direction.x * -1.0 * 0.04;
        let displacement = Vec3::new(0.0, polar, azimuthal);
        self.movement.set_target_relative_to_position(displacement);
    }

    pub(crate) fn stop(&mut self) {
        self.movement.remove_target();
    }
}
