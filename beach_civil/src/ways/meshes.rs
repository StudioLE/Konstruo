use beach_core::geometry::meshes::create_linestrip;
use bevy::math::vec3;
use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct WayMeshes {
    /// Linestrip mesh for the origin of a way control.
    pub control_origin: Handle<Mesh>,

    /// Linestrip mesh for the line from origin to handle of a way control.
    pub control_line: Handle<Mesh>,

    /// Linestrip mesh for the handle of a way control.
    pub control_handle: Handle<Mesh>,
}

pub fn insert_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let node = Cuboid::from_size(vec3(1.0, 1.0, 0.25));
    let line = vec![Vec3::ZERO, vec3(1.0, 0.0, 0.0)];
    commands.insert_resource(WayMeshes {
        control_origin: meshes.add(node),
        control_line: meshes.add(create_linestrip(line)),
        control_handle: meshes.add(node),
    });
}
