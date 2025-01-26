use bevy::prelude::*;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct BuildingMeshes {
    /// Linestrip mesh for the handle of a way control.
    pub module: Handle<Mesh>,
}

impl BuildingMeshes {
    /// System to insert [`BuildingMeshes`] on startup.
    pub(super) fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        let node = Cuboid::from_size(Vec3::new(1.0, 1.0, 1.0));
        commands.insert_resource(BuildingMeshes {
            module: meshes.add(node),
        });
    }
}
