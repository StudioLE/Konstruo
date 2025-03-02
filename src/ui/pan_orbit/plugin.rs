use super::*;
use crate::ui::PRIMARY_CAMERA_ORDER;
use bevy::prelude::*;

/// Plugin to enable the creation of [`Camera3d`] that is controller by [`Pan`] and [`Orbit`].
pub struct PanOrbitCameraPlugin;

impl Plugin for PanOrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, Pan::update_system)
            .add_systems(Update, Pan::orbit_changed_system)
            .add_systems(Update, Pan::keyboard_input_system)
            .add_systems(Update, Pan::mouse_button_input_system)
            .add_systems(Update, Pan::mouse_motion_input_system)
            .add_systems(Update, Orbit::update_system)
            .add_systems(Update, Orbit::keyboard_input_system)
            .add_systems(Update, Orbit::mouse_button_input_system)
            .add_systems(Update, Orbit::mouse_motion_input_system)
            .add_systems(Update, Orbit::scroll_wheel_input_system);
    }
}

fn spawn_camera(mut commands: Commands) {
    let pan = Pan::default();
    let transform = pan.get_transform();
    let bundle = (pan, transform);
    let pan = commands.spawn(bundle).id();
    let orbit = Orbit::default();
    let transform = orbit.get_cartesian_transform();
    let bundle = (
        PrimaryCamera,
        orbit,
        transform,
        Camera3d::default(),
        Camera {
            order: PRIMARY_CAMERA_ORDER,
            ..default()
        },
    );
    commands.spawn(bundle).set_parent(pan);
}
