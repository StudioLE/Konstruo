use crate::geometry::meshes::{create_triangle_list, create_triangular_prism};
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
        let prism = &create_triangular_prism();
        // let lines: Vec<[Vec3; 2]> = prism
        //     .map(|x| [[x[0], x[1]], [x[1], x[2]], [x[2], x[0]]])
        //     .iter()
        //     .flatten()
        //     .copied()
        //     .collect();
        // info!("{lines:?}");
        // let pitched = create_line_list(&lines);
        let pitched = create_triangle_list(prism);
        commands.insert_resource(BuildingMeshes {
            cuboid_module: meshes.add(cuboid),
            pitched_module: meshes.add(pitched),
        });
    }
}
