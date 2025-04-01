use crate::constraints::clamp_float::ClampFloat;
use crate::constraints::clamp_vec3::ClampVec3;
use crate::kinematics::Translation;
use crate::ui::pan_orbit::orbit::DEFAULT_RADIUS;
use crate::CAMERA_MAX;
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
                    x: ClampFloat::Fixed(-CAMERA_MAX, CAMERA_MAX),
                    y: ClampFloat::Fixed(-CAMERA_MAX, CAMERA_MAX),
                    z: ClampFloat::Fixed(-CAMERA_MAX, CAMERA_MAX),
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
    #[must_use]
    pub fn get_transform(&self) -> Transform {
        Transform::from_translation(self.translation.current)
    }

    /// Pan the camera in direction.
    pub(crate) fn in_direction(&mut self, direction: Vec3) {
        let velocity = direction * self.translation.speed;
        self.translation.set_target_relative_to_current(velocity);
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
