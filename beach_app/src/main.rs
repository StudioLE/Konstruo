use beach_civil::ways::{Way, WaysPlugin};
use beach_core::beziers::{CubicBezier, CubicBezierSpline};
use beach_geography::{GroundPlugin, SkyPlugin, SunPlugin};
use beach_ui::axis_marker::{AxisMarker, AxisMarkerPlugin};
use beach_ui::gizmos::GizmoPlugin;
use beach_ui::grid::GridPlugin;
use beach_ui::pan_orbit::PanOrbitCameraPlugin;
use beach_ui::view_cube::ViewCubePlugin;
use bevy::math::vec3;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AxisMarkerPlugin)
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
        .add_systems(Startup, spawn_positive_marker)
        .add_systems(Startup, spawn_way_example)
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

fn spawn_way_example(mut commands: Commands) {
    let curves = CubicBezierSpline {
        curves: vec![
            CubicBezier {
                start: vec3(0.0, 70.0, 0.0),
                start_handle: vec3(30.0, 70.0, 0.0),
                end_handle: vec3(30.0, 40.0, 0.0),
                end: vec3(50.0, 40.0, 0.0),
            },
            CubicBezier {
                start: vec3(50.0, 40.0, 0.0),
                start_handle: vec3(70.0, 40.0, 0.0),
                end_handle: vec3(70.0, 15.0, 0.0),
                end: vec3(70.0, 0.0, 0.0),
            },
        ],
    };
    let way = Way::new(curves);
    commands.spawn(way);
}
