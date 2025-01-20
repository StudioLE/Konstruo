use crate::cameras::primary_camera::PrimaryCamera;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use CursorPositionError::*;

#[derive(Debug)]
pub enum CursorPositionError {
    NoPrimaryWindow,
    NoPrimaryCamera,
    CursorOutsideWindow,
    InvalidRay,
    NoIntersection,
}

/// <https://bevy-cheatbook.github.io/cookbook/cursor2world.html>
pub fn get_cursor_position(
    window: &Query<&Window, With<PrimaryWindow>>,
    camera: &Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) -> Result<Vec3, CursorPositionError> {
    let window = window.get_single().map_err(|_| NoPrimaryWindow)?;
    let (camera, camera_transform) = camera.get_single().map_err(|_| NoPrimaryCamera)?;
    let cursor_position = window.cursor_position().ok_or(CursorOutsideWindow)?;
    let ray = camera
        .viewport_to_world(camera_transform, cursor_position)
        .map_err(|_| InvalidRay)?;
    let plane = InfinitePlane3d::new(Vec3::Z);
    let distance = ray
        .intersect_plane(Vec3::ZERO, plane)
        .ok_or(NoIntersection)?;
    Ok(ray.get_point(distance))
}
