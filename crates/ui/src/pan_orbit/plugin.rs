use super::*;
use bevy::prelude::*;

/// Plugin to enable the creation of [`Camera3d`] that is controller by [`Pan`] and [`Orbit`].
pub struct PanOrbitCameraPlugin;

impl Plugin for PanOrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_system)
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

/// System to spawn [`Pan`] and [`Orbit`] with a [`Camera`].
fn startup_system(mut commands: Commands) {
    commands.spawn(Pan::bundle()).with_child(Orbit::bundle());
}
