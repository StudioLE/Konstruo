use crate::beziers::{CubicBezier, CubicBezierSpline};
use crate::geometry::Vec6;
use crate::infrastructure::SurfaceType::*;
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
        let entity = way
            .clone()
            .spawn(&mut commands, &mut meshes, &way_meshes, &materials);
        let road = WaySurface::centered(4.8, 0.025, Carriageway);
        road.spawn(&mut commands, &mut meshes, &materials, &way, entity);
        let footway = WaySurface::new(Vec6::new(2.4, 4.4, 0.0, 0.0, 0.0, 0.125), Footway);
        footway.spawn(&mut commands, &mut meshes, &materials, &way, entity);
        let footway = WaySurface::new(Vec6::new(-4.4, -2.4, 0.0, 0.0, 0.0, 0.125), Footway);
        footway.spawn(&mut commands, &mut meshes, &materials, &way, entity);
    }
}
