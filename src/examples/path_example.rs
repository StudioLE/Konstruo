use crate::beziers::CubicBezierSpline;
use crate::infrastructure::*;
use bevy::prelude::*;

pub struct PathExample;

impl Plugin for PathExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl PathExample {
    fn startup_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<PathMaterials>,
        path_meshes: Res<PathMeshes>,
    ) {
        let spline = CubicBezierSpline::example();
        let path = Path::new(spline);
        let entity = path
            .clone()
            .spawn(&mut commands, &mut meshes, &path_meshes, &materials);
        for surface in PathSurface::default_surfaces() {
            surface.spawn(&mut commands, &mut meshes, &materials, &path, entity);
        }
    }
}
