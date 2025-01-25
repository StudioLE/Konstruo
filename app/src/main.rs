use bevy::math::vec3;
use bevy::prelude::*;
use geometrician_core::beziers::{CubicBezier, CubicBezierSpline};
use geometrician_environment::{GroundPlugin, SkyPlugin, SunPlugin};
use geometrician_infrastructure::ways::SurfaceType::*;
use geometrician_infrastructure::ways::{Way, WayMaterials, WaySurface, WaysPlugin};
use geometrician_ui::axis_marker::{AxisMarker, AxisMarkerPlugin};
use geometrician_ui::gizmos::GizmoPlugin;
use geometrician_ui::grid::GridPlugin;
use geometrician_ui::pan_orbit::PanOrbitCameraPlugin;
use geometrician_ui::view_cube::ViewCubePlugin;

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
        .add_systems(PostStartup, spawn_way_example)
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

fn spawn_way_example(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<WayMaterials>,
) {
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
    let road = WaySurface::centered(4.8, Carriageway);
    let entity = commands.spawn(way.clone()).id();
    road.spawn(&mut commands, &mut meshes, &materials, &way, entity);
    let footway = WaySurface::new([2.4, 4.4], Footway);
    footway.spawn(&mut commands, &mut meshes, &materials, &way, entity);
    let footway = WaySurface::new([-2.4, -4.4], Footway);
    footway.spawn(&mut commands, &mut meshes, &materials, &way, entity);
}
