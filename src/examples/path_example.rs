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
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: Res<PathMaterials>,
        path_meshes: Res<PathMeshes>,
    ) {
        let mut factory = PathFactory {
            commands,
            meshes,
            path_meshes,
            materials,
        };
        let spline = CubicBezierSpline::example();
        let path = Path::new(spline);
        let entity = factory.spawn_path(path.clone());
        for surface in PathSurface::default_surfaces() {
            factory.spawn_surface(surface, &path, entity);
        }
    }
}
