use crate::geometry::{Cuboid, TriangularPrism};
use crate::mathematics::HALF_PI;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct BuildingMeshes {
    /// Solid geometry of a [`BuildingModule`].
    pub cuboid: Handle<Mesh>,
    /// Solid geometry of a front to back pitched [`BuildingModule`].
    pub pitch_front_back: Handle<Mesh>,
    /// Solid geometry of a left to right pitched [`BuildingModule`].
    pub pitch_left_right: Handle<Mesh>,
    /// Edge geometry of a [`BuildingModule`].
    pub cuboid_edges: Handle<Mesh>,
    /// Edge geometry of a front to back pitched [`BuildingModule`].
    pub pitch_front_back_edges: Handle<Mesh>,
    /// Edge geometry of a left to right pitched [`BuildingModule`].
    pub pitch_left_right_edges: Handle<Mesh>,
}

impl BuildingMeshes {
    /// System to insert [`BuildingMeshes`] on startup.
    pub(super) fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        let cuboid = Cuboid::default();
        let pitched_front_back = TriangularPrism::default();
        let pitched_left_right = TriangularPrism::default().with_transform(
            Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, HALF_PI)),
        );
        let cuboid_edges = cuboid.clone().get_edges();
        let pitched_front_back_edges = pitched_front_back.clone().to_edges();
        let pitched_left_right_edges = pitched_left_right.clone().to_edges();
        commands.insert_resource(BuildingMeshes {
            cuboid: meshes.add(cuboid.get_triangles().to_mesh()),
            pitch_front_back: meshes.add(pitched_front_back.to_triangles().to_mesh()),
            pitch_left_right: meshes.add(pitched_left_right.to_triangles().to_mesh()),
            cuboid_edges: meshes.add(cuboid_edges.to_mesh()),
            pitch_front_back_edges: meshes.add(pitched_front_back_edges.to_mesh()),
            pitch_left_right_edges: meshes.add(pitched_left_right_edges.to_mesh()),
        });
    }
}
