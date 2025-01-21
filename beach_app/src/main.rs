use beach_civil::ways::ways_plugin;
use beach_geography::environment_plugin;
use beach_ui::axis_marker::{AxisMarker, AxisMarkerPlugin};
use beach_ui::grid::GridPlugin;
use beach_ui::pan_orbit::PanOrbitCameraPlugin;
use beach_ui::tools::tools_plugin;
use beach_ui::view_cube::ViewCubePlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AxisMarkerPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(GridPlugin)
        // .add_plugins(debug_plugin)
        .add_plugins(environment_plugin)
        .add_plugins(tools_plugin)
        .add_plugins(ViewCubePlugin)
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
