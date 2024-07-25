use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct Cursor {
    /// Current position in world space coordinates
    pub position: Vec3,

    /// Change in position since the last frame
    pub delta: Vec3,
}

pub fn update_cursor_position(
    cursor: ResMut<Cursor>,
    window: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    update_cursor_position_internal(cursor, window, cameras);
}

/// <https://bevy-cheatbook.github.io/cookbook/cursor2world.html>
pub fn update_cursor_position_internal(
    mut cursor: ResMut<Cursor>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut cameras: Query<(&Camera, &GlobalTransform)>,
) -> Option<()> {
    let plane = InfinitePlane3d::new(Vec3::Y);
    let ray = get_ray(window, &mut cameras)?;
    let distance = ray.intersect_plane(Vec3::ZERO, plane)?;
    let position = ray.get_point(distance);
    cursor.delta = position - cursor.position;
    cursor.position = position;
    Some(())
}

fn get_ray(
    window: Query<&Window, With<PrimaryWindow>>,
    cameras: &mut Query<(&Camera, &GlobalTransform)>
) -> Option<Ray3d> {
    let (camera, camera_transform) = cameras
        .iter_mut()
        .min_by_key(|(camera, _)| camera.order)?;
    let cursor_position = window.get_single().ok()?.cursor_position()?;
    camera.viewport_to_world(camera_transform, cursor_position)
}


