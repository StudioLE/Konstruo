use crate::constraints::clamp_float::ClampFloat;
use crate::constraints::clamp_vec3::ClampVec3;
use crate::geometry::Vec3Helpers;
use bevy::prelude::*;

/// A system to move the current translation to a target.
pub struct Translation {
    /// Current translation.
    pub current: Vec3,
    /// Target to move towards.
    pub target: Option<Vec3>,
    /// Constraints for the translation.
    pub clamp: ClampVec3,
    /// Speed in metres per second.
    pub speed: Vec3,
}

impl Translation {
    /// Is an update required?
    #[must_use]
    pub fn is_update_required(&self) -> bool {
        self.target.is_some()
    }

    /// Move the current translation towards the target for a single frame.
    ///
    /// Calling this method triggers a `Changed` event as it mutably borrows.
    /// To avoid this call [`Translation::is_update_required()`] first.
    /// - <https://bevy-cheatbook.github.io/programming/change-detection.html#what-gets-detected>
    pub fn update(&mut self) {
        let Some(target) = self.target else {
            warn!("An unnecessary mutable call was made to Translation::update()");
            return;
        };
        let total_displacement = self.get_displacement_to(target);
        if Vec3Helpers::is_almost_zero(total_displacement) {
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
            self.set_current(self.current + displacement);
        }
    }

    /// Set the current translation
    ///
    /// Translation is clamped to the constraints.
    ///
    /// Note: `set_translation` does not update the Transform
    pub fn set_current(&mut self, translation: Vec3) {
        self.current = self.clamp.clamp(translation);
    }

    /// Set the current translation by adding the displacement.
    ///
    /// Translation is clamped to the constraints.
    ///
    /// Note: `set_translation_relative` does not update the Transform
    pub fn set_current_relative(&mut self, displacement: Vec3) {
        self.set_current(self.current + displacement);
    }

    /// Set the target translation
    ///
    /// Translation is clamped to the constraints.
    pub fn set_target(&mut self, target: Vec3) {
        let target = self.clamp.clamp(target);
        if Vec3Helpers::is_almost_equal_to(self.current, target) {
            self.remove_target();
        } else {
            self.target = Some(target);
        }
    }

    /// Set the target translation by combining the current translation and the displacement.
    pub fn set_target_relative_to_current(&mut self, displacement: Vec3) {
        self.set_target(self.current + displacement);
    }

    /// Set the target translation by adding the displacement.
    pub fn set_target_relative(&mut self, displacement: Vec3) {
        if let Some(target) = self.target {
            self.set_target(target + displacement);
        } else {
            self.set_target(self.current + displacement);
        }
    }

    /// Remove the target translation.
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
        adjust(&mut displacement.x, &self.clamp.x);
        adjust(&mut displacement.y, &self.clamp.y);
        adjust(&mut displacement.z, &self.clamp.z);
        displacement
    }
}

fn adjust(displacement: &mut f32, clamp: &ClampFloat) {
    if let ClampFloat::Wrapped(max) = clamp {
        if displacement.abs() > (max / 2.0) {
            *displacement *= -1.0;
        }
    }
}
