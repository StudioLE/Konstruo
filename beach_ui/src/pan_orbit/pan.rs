use crate::pan_orbit::orbit::DEFAULT_RADIUS;
use beach_core::constraints::clamp_float::ClampFloat;
use beach_core::constraints::clamp_vec3::ClampVec3;
use beach_core::kinematics::Translation;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub(super) const SPEED_MODIFIER: f32 = 2.0;

/// 2D translation of the [`PanOrbitCameraPlugin`] on the XY plane.
///
/// The [`Orbit`] entity is a child of the [`Pan`] entity.
/// Therefore any translation of [`Pan`] will affect [`Orbit`].
///
/// Cartesian coordiantes are used.
#[derive(Component)]
#[require(InheritedVisibility)]
pub struct Pan {
    /// Translation on the XY plane.
    ///
    /// Cartesian coordinates are used.
    pub(super) translation: Translation,
    /// Is dragging mode currently active?
    ///
    /// The value is the cursor translation on the XY plane when dragging was started.
    pub(super) dragging: Option<Vec3>,
}

impl Default for Pan {
    fn default() -> Self {
        Self {
            translation: Translation {
                current: Vec3::ZERO,
                clamp: ClampVec3 {
                    x: ClampFloat::Fixed(-1000.0, 1000.0),
                    y: ClampFloat::Fixed(-1000.0, 1000.0),
                    z: ClampFloat::Fixed(-1000.0, 1000.0),
                },
                target: None,
                speed: Vec3::splat(DEFAULT_RADIUS * SPEED_MODIFIER),
            },
            dragging: None,
        }
    }
}

impl Pan {
    /// Get the current translation as a transform.
    pub fn get_transform(&self) -> Transform {
        Transform::from_translation(self.translation.current)
    }

    /// Pan the camera in direction.
    pub(crate) fn in_direction(&mut self, direction: Vec3) {
        let velocity = direction * self.translation.speed;
        self.translation.set_target_relative_to_current(velocity);
    }

    /// Pan the camera in the direction of the mouse motion.
    #[allow(dead_code)]
    fn in_direction_of_motion(&mut self, mouse: MouseMotion) {
        let direction = mouse.delta.normalize();
        let polar = direction.y * -1.0 * 0.1;
        let azimuthal = direction.x * -1.0 * 0.04;
        let displacement = Vec3::new(0.0, polar, azimuthal);
        self.translation
            .set_target_relative_to_current(displacement);
    }

    /// Pan the camera by translation from the start of the drag
    pub(crate) fn by_grab(&mut self, mut transform: Mut<Transform>, cursor: Vec3) {
        let Some(start) = self.dragging else {
            warn!("Failed to get drag origin");
            return;
        };
        let translation = start - cursor;
        self.translation.set_target_relative_to_current(translation);
        let target = self.translation.current + translation;
        *transform = Transform::from_translation(target);
    }

    pub(crate) fn stop(&mut self) {
        self.translation.remove_target();
    }
}
