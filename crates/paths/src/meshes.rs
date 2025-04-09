use bevy::prelude::*;
use konstruo_core::QUARTER_PI;
use konstruo_geometry::Cuboid;

#[allow(clippy::struct_field_names)]
#[derive(Resource)]
pub struct PathMeshes {
    /// Mesh for the origin of a path control.
    pub control_origin: Handle<Mesh>,
    /// Mesh for the handle of a path control.
    pub control_handle: Handle<Mesh>,
}

impl PathMeshes {
    /// System to insert [`PathMeshes`] on startup.
    pub(super) fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        let scale = Transform::from_scale(Vec3::new(1.500, 1.500, 0.500));
        let origin = Cuboid::new(scale.with_rotation(Quat::from_rotation_z(QUARTER_PI)));
        let handle = Cuboid::new(scale);
        commands.insert_resource(PathMeshes {
            control_origin: meshes.add(origin.get_triangles().to_mesh()),
            control_handle: meshes.add(handle.get_triangles().to_mesh()),
        });
    }
}
