use crate::beziers::CubicBezierSpline;
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
        let spline = CubicBezierSpline::example();
        let way = Way::new(spline);
        let entity = way
            .clone()
            .spawn(&mut commands, &mut meshes, &way_meshes, &materials);
        for surface in WaySurface::default_surfaces() {
            surface.spawn(&mut commands, &mut meshes, &materials, &way, entity);
        }
    }
}
