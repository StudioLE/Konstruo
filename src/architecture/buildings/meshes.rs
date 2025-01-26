use crate::geometry::TriangularPrism;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct BuildingMeshes {
    /// A cuboid mesh with 1.0 length edges centered at the origin.
    pub cuboid_module: Handle<Mesh>,
    /// A trianglular prism mesh with 1.0 lengths and height centered at the origin.
    pub pitched_module: Handle<Mesh>,
}

impl BuildingMeshes {
    /// System to insert [`BuildingMeshes`] on startup.
    pub(super) fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        let cuboid = Cuboid::from_size(Vec3::new(1.0, 1.0, 1.0));
        let pitched = TriangularPrism::default().to_triangle_list().to_mesh();
        commands.insert_resource(BuildingMeshes {
            cuboid_module: meshes.add(cuboid),
            pitched_module: meshes.add(pitched),
        });
    }
}
