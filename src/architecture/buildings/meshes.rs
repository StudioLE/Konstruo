use crate::geometry::TriangularPrism;
use crate::mathematics::HALF_PI;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct BuildingMeshes {
    /// A cuboid mesh with 1.0 length edges centered at the origin.
    pub cuboid_module: Handle<Mesh>,
    /// A trianglular prism mesh with 1.0 lengths and height centered at the origin.
    pub pitched_front_back_module: Handle<Mesh>,
    /// A trianglular prism mesh with 1.0 lengths and height centered at the origin.
    pub pitched_left_right_module: Handle<Mesh>,
}

impl BuildingMeshes {
    /// System to insert [`BuildingMeshes`] on startup.
    pub(super) fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        let cuboid = Cuboid::from_size(Vec3::new(1.0, 1.0, 1.0));
        let pitched_front_back = TriangularPrism::default().to_triangle_list();
        let pitched_left_right =
            pitched_front_back
                .clone()
                .with_transform(Transform::from_rotation(Quat::from_axis_angle(
                    Vec3::Z,
                    HALF_PI,
                )));
        commands.insert_resource(BuildingMeshes {
            cuboid_module: meshes.add(cuboid),
            pitched_front_back_module: meshes.add(pitched_front_back.to_mesh()),
            pitched_left_right_module: meshes.add(pitched_left_right.to_mesh()),
        });
    }
}
