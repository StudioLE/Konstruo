use crate::beziers::{CubicBezier, CubicBezierSpline};
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
}
