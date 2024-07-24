use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct Cursor {
    /// Current position in world space coordinates
    pub position: Vec3,

    /// Change in position since the last frame
    pub delta: Vec3,
}

/// <https://bevy-cheatbook.github.io/cookbook/cursor2world.html>
pub fn update_cursor_position(
    mut cursor: ResMut<Cursor>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let Some((camera, camera_transform)) =
        cameras.iter_mut().min_by_key(|(camera, _)| camera.order)
    else {
        return;
    };
    let window = q_window.single();
    let Some(cursor_position) = window.cursor_position() else {
        // if the cursor is not inside the window, we can't do anything
        return;
    };
    let plane = InfinitePlane3d::new(Vec3::Y);
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        // if it was impossible to compute for whatever reason; we can't do anything
        return;
    };
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, plane) else {
        return;
    };
    let position = ray.get_point(distance);
    cursor.delta = position - cursor.position;
    cursor.position = position;
}
