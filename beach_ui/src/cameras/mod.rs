pub mod orbit;
mod pan;

use crate::cameras::orbit::Orbit;
use crate::cameras::pan::Pan;
use bevy::prelude::*;

/// Register systems for cameras.
pub fn cameras_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(Update, pan::on_update)
        .add_systems(Update, pan::on_changed)
        .add_systems(Update, pan::on_input)
        .add_systems(Update, orbit::on_update)
        .add_systems(Update, orbit::on_changed)
        .add_systems(Update, orbit::on_input);
}

fn spawn_camera(mut commands: Commands) {
    let pan = Pan::default();
    let bundle = SpatialBundle {
        transform: pan.get_transform(),
        ..Default::default()
    };
    let pan = commands.spawn((bundle, pan)).id();
    let orbit = Orbit::default();
    // let mut orbit = OrbitState::default();
    // let target = Vec3::new(100.0, 0.0, QUARTER_PI);
    // orbit.movement.set_target(target);
    let bundle = Camera3dBundle {
        transform: orbit.get_transform(),
        ..Default::default()
    };
    commands.spawn((bundle, orbit)).set_parent(pan);
}
