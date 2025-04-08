use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct PathMeshes {
    /// Linestrip mesh for the origin of a path control.
    pub control_origin: Handle<Mesh>,

    /// Linestrip mesh for the handle of a path control.
    pub control_handle: Handle<Mesh>,
}

impl PathMeshes {
    /// System to insert [`PathMeshes`] on startup.
    pub(super) fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        let node = Cuboid::from_size(Vec3::new(1.500, 1.500, 0.500));
        commands.insert_resource(PathMeshes {
            control_origin: meshes.add(node),
            control_handle: meshes.add(node),
        });
    }
}
