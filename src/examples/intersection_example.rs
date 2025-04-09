use crate::examples::{ExampleFactory, ExampleMaterials, PathExample};
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use konstruo_beziers::{CubicBezier, CubicBezierSpline};
use konstruo_distribution::*;
use konstruo_paths::{Path, PathFactory, PathMaterials, PathMeshes, PathSurface};

pub struct IntersectionExample;

impl Plugin for IntersectionExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl IntersectionExample {
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
        let spline = spline_a();
        let splines = spline.split_at_param(0.5).expect("should be valid");

        for spline in splines {
            let path = Path::new(spline);
            let entity = factory.spawn_path(path.clone());
            for surface in PathSurface::default_surfaces() {
                factory.spawn_surface(surface, &path, entity);
            }
        }
    }
}

pub fn spline_a() -> CubicBezierSpline {
    CubicBezierSpline::new(vec![CubicBezier::new(
        Vec3::new(0.0, -100.0, 0.0),
        Vec3::new(0.0, -50.0, 0.0),
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(100.0, 0.0, 0.0),
    )
    .expect("should be valid")])
    .expect("should be valid")
}
