use crate::examples::ExampleMaterials;
use bevy::prelude::*;
use konstruo_core::{Vec3Extensions, VecVec3Extensions};
use konstruo_geometry::*;

pub struct SubdivisionExample;

impl Plugin for SubdivisionExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl SubdivisionExample {
    fn startup_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let subdivision = Subdivision::example();
        let regions = subdivision.execute().expect("should be valid");
        for region in regions {
            spawn_rectangle(&mut commands, &mut meshes, &mut materials, region);
        }
    }
}

fn spawn_rectangle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    rectangle: [Vec3; 4],
) {
    let triangles = Triangle::from_rectangle(rectangle);
    let is_ccw = rectangle.is_ccw(Vec3::Z).expect("should be valid");
    let material = if is_ccw {
        ExampleMaterials::blue_face_transparent()
    } else {
        ExampleMaterials::red_face()
    };
    let bundle = (
        Solid,
        MeshMaterial3d(materials.add(material)),
        Mesh3d(meshes.add(TriangleList::new(triangles.to_vec()).to_mesh())),
    );
    commands.spawn(bundle);
    for triangle in triangles {
        let material = if is_ccw {
            ExampleMaterials::blue_edge()
        } else {
            ExampleMaterials::red_edge()
        };
        let lines = LineList::from_lines(triangle.to_lines().to_vec());
        let bundle = (
            Wireframe,
            MeshMaterial3d(materials.add(material)),
            Mesh3d(meshes.add(lines.to_mesh())),
        );
        commands.spawn(bundle);
    }
}
