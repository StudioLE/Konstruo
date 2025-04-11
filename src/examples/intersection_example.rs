use bevy::prelude::*;
use konstruo_beziers::constants::{INTERSECTION_ACCURACY, INTERSECTION_TOLERANCE, OFFSET_ACCURACY};
use konstruo_beziers::{CubicBezier, CubicBezierSpline};
use konstruo_paths::{Path, PathFactory, PathIntersectionBuilder, PathMaterials, PathMeshes};

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
        let mut splines = spline
            .split_at_param(0.5)
            .expect("should be valid")
            .to_vec();
        splines.push(spline_b());
        let mut intersection = PathIntersectionBuilder::default();
        for spline in splines {
            let path = Path::new(spline);
            let entity = factory.spawn_path(path.clone());
            factory
                .commands
                .entity(entity)
                .insert(MeshMaterial3d(factory.materials.control_node_drag.clone()));
            // for surface in PathSurface::default_surfaces() {
            //     factory.spawn_surface(surface, &path, entity);
            // }
            intersection.add(entity, path.spline.clone());
        }
        let intersection = intersection.build().expect("should be valid");

        let corners = intersection.get_corners();
        for spline in corners {
            let offset = spline
                .offset_without_intersection(
                    2.5,
                    OFFSET_ACCURACY,
                    INTERSECTION_TOLERANCE,
                    INTERSECTION_ACCURACY,
                )
                .expect("should be valid");
            let path = Path::new(offset);
            let _ = factory.spawn_path(path.clone());
        }
    }
}

fn spline_a() -> CubicBezierSpline {
    CubicBezierSpline::new(vec![CubicBezier::new(
        Vec3::new(0.0, -100.0, 0.0),
        Vec3::new(0.0, -50.0, 0.0),
        Vec3::new(50.0, 0.0, 0.0),
        Vec3::new(100.0, 0.0, 0.0),
    )
    .expect("should be valid")])
    .expect("should be valid")
}

fn spline_b() -> CubicBezierSpline {
    CubicBezierSpline::new(vec![CubicBezier::new(
        Vec3::new(31.25, -31.25, 0.0),
        Vec3::new(50.0, -50.0, 0.0),
        Vec3::new(75.0, -75.0, 0.0),
        Vec3::new(100.0, -100.0, 0.0),
    )
    .expect("should be valid")])
    .expect("should be valid")
}
