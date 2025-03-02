use bevy::prelude::*;
use konstruo::architecture::*;
use konstruo::distribution::DistributionPlugin;
use konstruo::environment::*;
use konstruo::examples::*;
use konstruo::infrastructure::*;
use konstruo::ui::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AxisMarkerPlugin)
        .add_plugins(BuildingsPlugin)
        // .add_plugins(CursorGizmoPlugin)
        .add_plugins(DistributionPlugin)
        .add_plugins(FloatingActionPlugin)
        .add_plugins(GizmoPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(OriginMarkerPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(SkyPlugin)
        .add_plugins(SunPlugin)
        .add_plugins(ViewCubePlugin)
        // .add_plugins(debug_plugin)
        .add_plugins(WaysPlugin)
        // .add_plugins(AxisMarkerExample)
        // .add_plugins(BuildingsExample)
        // .add_plugins(FlexAlongBezierExample)
        .add_plugins(FlexBuildingsAlongWayExample)
        // .add_plugins(FlexNestedExample)
        // .add_plugins(FlexVerticallyExample)
        // .add_plugins(FlexWrappingExample)
        .add_plugins(WayExample)
        .run();
}
