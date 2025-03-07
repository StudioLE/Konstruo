use crate::beziers::{CubicBezier, CubicBezierSpline};
use crate::infrastructure::*;
use bevy::prelude::*;

pub struct WayExample;

impl Plugin for WayExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl WayExample {
    fn startup_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<WayMaterials>,
        way_meshes: Res<WayMeshes>,
    ) {
        let spline = CubicBezierSpline::new(vec![
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
        ]);
        let way = Way::new(spline);
        let entity = way
            .clone()
            .spawn(&mut commands, &mut meshes, &way_meshes, &materials);
        for surface in WaySurface::default_surfaces() {
            surface.spawn(&mut commands, &mut meshes, &materials, &way, entity);
        }
    }
}
