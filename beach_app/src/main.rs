use beach_civil::ways::ways_plugin;
use beach_geography::environment_plugin;
use beach_ui::axis_marker::axis_marker_plugin;
pub use beach_ui::cameras::cameras_plugin;
use beach_ui::tools::tools_plugin;
use beach_ui::view_cube::view_cube_plugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(axis_marker_plugin)
        .add_plugins(cameras_plugin)
        .add_plugins(environment_plugin)
        .add_plugins(tools_plugin)
        .add_plugins(view_cube_plugin)
        .add_plugins(ways_plugin)
        .run();
}
