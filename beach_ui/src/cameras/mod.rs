pub mod orbit;
mod orbit_input;
mod orbit_update;
mod pan;
mod pan_input;
mod pan_update;
pub mod primary_camera;

use crate::cameras::orbit::Orbit;
use crate::cameras::pan::Pan;
use crate::cameras::primary_camera::PrimaryCamera;
use bevy::prelude::*;

/// Register systems for cameras.
pub fn cameras_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(Update, Pan::update_system)
        .add_systems(Update, Pan::input_system)
        .add_systems(Update, Orbit::update_system)
        .add_systems(Update, Orbit::input_system);
}

fn spawn_camera(mut commands: Commands) {
    let pan = Pan::default();
    let transform = pan.get_transform();
    let bundle = (pan, transform);
    let pan = commands.spawn(bundle).id();
    let orbit = Orbit::default();
    let transform = orbit.get_transform();
    let bundle = (PrimaryCamera, orbit, transform, Camera3d::default());
    commands.spawn(bundle).set_parent(pan);
}
