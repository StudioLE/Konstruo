use beach::architecture::*;
use beach::environment::*;
use beach::examples::*;
use beach::infrastructure::*;
use beach::ui::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AxisMarkerPlugin)
        .add_plugins(BuildingsPlugin)
        // .add_plugins(CursorGizmoPlugin)
        .add_plugins(GizmoPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(SkyPlugin)
        .add_plugins(SunPlugin)
        .add_plugins(ViewCubePlugin)
        // .add_plugins(debug_plugin)
        .add_plugins(WaysPlugin)
        .add_plugins(AxisMarkerExample)
        // .add_plugins(BuildingsExample)
        .add_plugins(DistributionExample)
        .add_plugins(WayExample)
        .run();
}
