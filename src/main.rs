use beach::architecture::*;
use beach::beziers::CubicBezier;
use beach::beziers::CubicBezierSpline;
use beach::environment::{GroundPlugin, SkyPlugin, SunPlugin};
use beach::infrastructure::SurfaceType::{Carriageway, Footway};
use beach::infrastructure::{Way, WayMaterials, WaySurface, WaysPlugin};
use beach::ui::GizmoPlugin;
use beach::ui::GridPlugin;
use beach::ui::PanOrbitCameraPlugin;
use beach::ui::ViewCubePlugin;
use beach::ui::{AxisMarker, AxisMarkerPlugin};
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
        .add_systems(Startup, spawn_positive_marker)
        .add_systems(PostStartup, spawn_buildings)
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
                start: Vec3::new(0.0, 70.0, 0.0),
                start_handle: Vec3::new(30.0, 70.0, 0.0),
                end_handle: Vec3::new(30.0, 40.0, 0.0),
                end: Vec3::new(50.0, 40.0, 0.0),
            },
            CubicBezier {
                start: Vec3::new(50.0, 40.0, 0.0),
                start_handle: Vec3::new(70.0, 40.0, 0.0),
                end_handle: Vec3::new(70.0, 15.0, 0.0),
                end: Vec3::new(70.0, 0.0, 0.0),
            },
        ],
    };
    let way = Way::new(curves);
    let road = WaySurface::centered(0.025, 4.8, Carriageway);
    let entity = commands.spawn(way.clone()).id();
    road.spawn(&mut commands, &mut meshes, &materials, &way, entity);
    let footway = WaySurface::new(0.125, [2.4, 4.4], Footway);
    footway.spawn(&mut commands, &mut meshes, &materials, &way, entity);
    let footway = WaySurface::new(0.125, [-2.4, -4.4], Footway);
    footway.spawn(&mut commands, &mut meshes, &materials, &way, entity);
}

fn spawn_buildings(
    mut commands: Commands,
    meshes: Res<BuildingMeshes>,
    materials: Res<BuildingMaterials>,
) {
    let stacks = vec![
        BuildingModuleStack {
            definition: BuildingModule {
                width: 4.8,
                length: 9.0,
                front_offset: 0.0,
                back_offset: 0.0,
                ..default()
            },
            levels: 2,
            level_height: 2.4,
        },
        BuildingModuleStack {
            definition: BuildingModule {
                width: 6.0,
                length: 5.4,
                front_offset: 3.0,
                back_offset: 0.6,
                ..default()
            },
            levels: 1,
            level_height: 2.4,
        },
    ];

    ModularBuildingFactory::spawn(&mut commands, &meshes, &materials, stacks)
        .expect("spawn should not fail");
}
