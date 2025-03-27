use crate::geometry::{TriangularPrism, Cuboid};
use crate::mathematics::HALF_PI;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct BuildingMeshes {
    /// A cuboid mesh with 1.0 length edges centered at the origin.
    pub cuboid: Handle<Mesh>,
    /// A trianglular prism mesh with 1.0 lengths and height centered at the origin.
    pub pitched_front_back: Handle<Mesh>,
    /// A trianglular prism mesh with 1.0 lengths and height centered at the origin.
    pub pitched_left_right: Handle<Mesh>,
}

impl BuildingMeshes {
    /// System to insert [`BuildingMeshes`] on startup.
    pub(super) fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        let cuboid = Cuboid::default();
        let pitched_front_back = TriangularPrism::default();
        let pitched_left_right = TriangularPrism::default().with_transform(
            Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, HALF_PI)),
        );
        commands.insert_resource(BuildingMeshes {
            cuboid: meshes.add(cuboid.to_triangles().to_mesh()),
            pitched_front_back: meshes.add(pitched_front_back.to_triangles().to_mesh()),
            pitched_left_right: meshes.add(pitched_left_right.to_triangles().to_mesh()),
        });
    }
}
