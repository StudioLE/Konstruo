use crate::examples::ExampleMaterials;
use crate::geometry::{Cuboid, Edge, Solid};
use bevy::prelude::*;
use crate::mathematics::QUARTER_PI;

pub struct Shapes3DExample;

impl Plugin for Shapes3DExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl Shapes3DExample {
    fn startup_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let transform = Transform::from_translation(Vec3::new(3.0, 2.0, 1.0))
            .with_scale(Vec3::new(3.0, 2.0, 1.0))
            .with_rotation(Quat::from_axis_angle(Vec3::Z, QUARTER_PI));
        let cuboid = Cuboid::new(transform);
        let bundle = (
            Solid,
            MeshMaterial3d(materials.add(ExampleMaterials::blue_face())),
            Mesh3d(meshes.add(cuboid.get_triangles().to_mesh())),
        );
        commands.spawn(bundle);
        let bundle = (
            Edge,
            MeshMaterial3d(materials.add(ExampleMaterials::blue_edge())),
            Mesh3d(meshes.add(cuboid.get_edges().to_mesh())),
        );
        commands.spawn(bundle);
    }
}
