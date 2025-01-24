use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct WayMeshes {
    /// Linestrip mesh for the origin of a way control.
    pub control_origin: Handle<Mesh>,

    /// Linestrip mesh for the handle of a way control.
    pub control_handle: Handle<Mesh>,
}

impl WayMeshes {
    /// System to insert [`WayMeshes`] on startup.
    pub(super) fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        let node = Cuboid::from_size(Vec3::new(1.0, 1.0, 0.25));
        commands.insert_resource(WayMeshes {
            control_origin: meshes.add(node),
            control_handle: meshes.add(node),
        });
    }
}
