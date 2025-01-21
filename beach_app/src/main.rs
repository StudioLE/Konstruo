use beach_civil::ways::ways_plugin;
use beach_geography::environment_plugin;
use beach_ui::axis_marker::{AxisMarker, AxisMarkerPlugin};
pub use beach_ui::cameras::cameras_plugin;
use beach_ui::tools::tools_plugin;
use beach_ui::view_cube::view_cube_plugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AxisMarkerPlugin)
        .add_plugins(cameras_plugin)
        // .add_plugins(debug_plugin)
        .add_plugins(environment_plugin)
        .add_plugins(tools_plugin)
        .add_plugins(view_cube_plugin)
        .add_plugins(ways_plugin)
        .add_systems(Startup, spawn_positive_marker)
        .run();
}

fn spawn_positive_marker(mut commands: Commands) {
    let bundle = (
        AxisMarker {
            thickness: 1.0,
            length: 10.0,
        },
        Transform::from_translation(Vec3::splat(10.0)),
    );
    commands.spawn(bundle);
}
