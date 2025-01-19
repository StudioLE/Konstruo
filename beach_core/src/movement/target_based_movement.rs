use crate::constraints::clamp_float::ClampFloat;
use crate::constraints::clamp_vec3::ClampVec3;
use crate::geometry::vectors::{is_almost_equal_to, is_almost_zero};
use bevy::prelude::*;

/// Move towards a target position.
pub struct TargetBasedMovement {
    /// Current position.
    pub current: Vec3,
    /// Target position to move towards.
    pub target: Option<Vec3>,
    /// Constraints for the position.
    pub clamp: ClampVec3,
    /// Speed in metres per second.
    pub speed: Vec3,
}

impl TargetBasedMovement {
    /// Move the current position towards the target for a single frame.
    pub fn update(&mut self) {
        let Some(target) = self.target else {
            return;
        };
        let total_displacement = self.get_displacement_to(target);
        if is_almost_zero(total_displacement) {
            self.current = target;
            self.remove_target();
            return;
        }
        let direction = total_displacement.normalize();
        let displacement = direction * self.get_speed_per_frame();
        if displacement.length() >= total_displacement.length() {
            self.current = target;
            self.remove_target();
        } else {
            self.set_position(self.current + displacement);
        }
    }

    /// Set the current position
    ///
    /// Position is clamped to the constraints.
    pub fn set_position(&mut self, position: Vec3) {
        self.current = self.clamp.clamp(position);
    }

    /// Set the target position
    ///
    /// Position is clamped to the constraints.
    pub fn set_target(&mut self, target: Vec3) {
        let target = self.clamp.clamp(target);
        if is_almost_equal_to(self.current, target) {
            self.remove_target();
        } else {
            self.target = Some(target);
        }
    }

    /// Set the target position by adding the displacement to the current position.
    pub fn set_target_relative_to_position(&mut self, displacement: Vec3) {
        self.set_target(self.current + displacement);
    }

    /// Set the target position by adding the displacement to the target position.
    pub fn set_target_relative(&mut self, displacement: Vec3) {
        if let Some(target) = self.target {
            self.set_target(target + displacement);
        } else {
            self.set_target(self.current + displacement);
        }
    }

    /// Remove the target position.
    pub fn remove_target(&mut self) {
        self.target = None;
    }

    /// Get the speed in metres per frame.
    fn get_speed_per_frame(&self) -> Vec3 {
        // TODO: Get current FPS
        let fps = 60.0;
        self.speed / fps
    }

    /// Get the total displacement from current to the target
    ///
    /// If an axis is wrapped then take in to account it may be quicker in the other direction
    fn get_displacement_to(&self, target: Vec3) -> Vec3 {
        let mut displacement = target - self.current;
        fn adjust(displacement: &mut f32, clamp: &ClampFloat) {
            if let ClampFloat::Wrapped(max) = clamp {
                if displacement.abs() > (max / 2.0) {
                    *displacement *= -1.0;
                }
            }
        }
        adjust(&mut displacement.x, &self.clamp.x);
        adjust(&mut displacement.y, &self.clamp.y);
        adjust(&mut displacement.z, &self.clamp.z);
        displacement
    }
}
