use crate::ui::PrimaryCamera;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use CursorPositionError::*;

pub struct Cursor;

#[derive(Debug)]
pub enum CursorPositionError {
    NoPrimaryWindow,
    NoPrimaryCamera,
    CursorOutsideWindow,
    InvalidRay,
    NoIntersection,
}

impl Cursor {
    /// Get the position of the cursor on the XY plane.
    /// <https://bevy-cheatbook.github.io/cookbook/cursor2world.html>
    pub fn from_position(
        camera: &Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
        position: Vec2,
    ) -> Result<Vec3, CursorPositionError> {
        let plane = InfinitePlane3d::new(Vec3::Z);
        let ray = get_ray_from_position(camera, position)?;
        let distance = ray
            .intersect_plane(Vec3::ZERO, plane)
            .ok_or(NoIntersection)?;
        Ok(ray.get_point(distance))
    }

    /// Get the position of the cursor on the XY plane.
    /// <https://bevy-cheatbook.github.io/cookbook/cursor2world.html>
    pub fn from_window(
        window: &Query<&Window, With<PrimaryWindow>>,
        camera: &Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) -> Result<Vec3, CursorPositionError> {
        let position = get_position_from_window(window)?;
        Self::from_position(camera, position)
    }
}

/// Get the 2d cursor position in the [`Window`].
fn get_position_from_window(
    window: &Query<&Window, With<PrimaryWindow>>,
) -> Result<Vec2, CursorPositionError> {
    let window = window.single().map_err(|_| NoPrimaryWindow)?;
    window.cursor_position().ok_or(CursorOutsideWindow)
}

/// Get the ray projection from the cursor in the direction of the camera.
fn get_ray_from_position(
    camera: &Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    position: Vec2,
) -> Result<Ray3d, CursorPositionError> {
    let (camera, camera_transform) = camera.single().map_err(|_| NoPrimaryCamera)?;
    camera
        .viewport_to_world(camera_transform, position)
        .map_err(|_| InvalidRay)
}
