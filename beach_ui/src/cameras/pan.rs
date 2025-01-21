use beach_core::constraints::clamp_float::ClampFloat;
use beach_core::constraints::clamp_vec3::ClampVec3;
use beach_core::movement::direct::DirectMovement;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

/// Pan state of the camera.
#[derive(Component)]
#[require(InheritedVisibility)]
pub struct Pan {
    pub movement: DirectMovement,
    /// Is dragging mode currently active?
    /// The value is the cursor position on the XY plane when dragging was started.
    pub dragging: Option<Vec3>,
}

impl Default for Pan {
    fn default() -> Self {
        Self {
            movement: DirectMovement {
                current: Vec3::ZERO,
                clamp: ClampVec3 {
                    x: ClampFloat::Fixed(-1000.0, 1000.0),
                    y: ClampFloat::Fixed(-1000.0, 1000.0),
                    z: ClampFloat::Fixed(-1000.0, 1000.0),
                },
                target: None,
                speed: Vec3::splat(500.0),
            },
            dragging: None,
        }
    }
}

impl Pan {
    /// Get the position.
    pub fn get_transform(&self) -> Transform {
        Transform::from_translation(self.movement.current)
    }

    /// Orbit the camera in direction relative to the Azimuth.
    pub(crate) fn in_direction(&mut self, direction: Vec3, modifier: f32) {
        let velocity = direction * self.movement.speed * modifier;
        self.movement.set_target_relative_to_position(velocity);
    }

    /// Orbit the camera in the direction of the mouse motion.
    #[allow(dead_code)]
    fn in_direction_of_motion(&mut self, mouse: MouseMotion) {
        let direction = mouse.delta.normalize();
        let polar = direction.y * -1.0 * 0.1;
        let azimuthal = direction.x * -1.0 * 0.04;
        let displacement = Vec3::new(0.0, polar, azimuthal);
        self.movement.set_target_relative_to_position(displacement);
    }

    pub(crate) fn by_grab(&mut self, mut transform: Mut<Transform>, cursor_position: Vec3) {
        let Some(drag_origin) = self.dragging else {
            warn!("Failed to get drag origin");
            return;
        };
        let translation = drag_origin - cursor_position;
        self.movement.set_target_relative_to_position(translation);
        let target = self.movement.current + translation;
        *transform = Transform::from_translation(target);
    }

    pub(crate) fn stop(&mut self) {
        self.movement.remove_target();
    }
}
