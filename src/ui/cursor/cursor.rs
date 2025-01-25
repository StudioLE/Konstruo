use crate::ui::pan_orbit::PrimaryCamera;
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
    pub fn on_ground(
        window: &Query<&Window, With<PrimaryWindow>>,
        camera: &Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) -> Result<Vec3, CursorPositionError> {
        let ray = Self::get_ray(window, camera)?;
        let plane = InfinitePlane3d::new(Vec3::Z);
        let distance = ray
            .intersect_plane(Vec3::ZERO, plane)
            .ok_or(NoIntersection)?;
        Ok(ray.get_point(distance))
    }

    /// Get the ray projection from the cursor in the direction of the camera.
    pub fn get_ray(
        window: &Query<&Window, With<PrimaryWindow>>,
        camera: &Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    ) -> Result<Ray3d, CursorPositionError> {
        let window = window.get_single().map_err(|_| NoPrimaryWindow)?;
        let (camera, camera_transform) = camera.get_single().map_err(|_| NoPrimaryCamera)?;
        let cursor_position = window.cursor_position().ok_or(CursorOutsideWindow)?;
        camera
            .viewport_to_world(camera_transform, cursor_position)
            .map_err(|_| InvalidRay)
    }
}
