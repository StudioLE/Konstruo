use crate::axis_marker::AxisMarker;
use crate::view_cube::camera::ViewCubeCamera;
use crate::view_cube::corner::ViewCubeCorner;
use crate::view_cube::edge::ViewCubeEdge;
use crate::view_cube::face::ViewCubeFace;
use crate::view_cube::materials::ViewCubeMaterials;
use crate::view_cube::meshes::ViewCubeMeshes;
use bevy::app::{App, Plugin, PostStartup, Startup, Update};
use bevy::prelude::{Commands, MeshPickingPlugin};
use bevy::render::view::{Layer, RenderLayers};

pub(super) const RENDER_LAYER: Layer = 1;

/// Plugin to display an interactive view cube that is rotated according to [`Orbit`].
pub struct ViewCubePlugin;

impl Plugin for ViewCubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_axis_marker)
            .add_systems(Startup, ViewCubeMaterials::startup_system)
            .add_systems(Startup, ViewCubeMeshes::startup_system)
            .add_systems(Startup, ViewCubeCamera::startup_system)
            .add_systems(PostStartup, ViewCubeFace::startup_system)
            .add_systems(PostStartup, ViewCubeEdge::startup_system)
            .add_systems(PostStartup, ViewCubeCorner::startup_system)
            .add_systems(Update, ViewCubeCamera::update_system)
            .add_plugins(MeshPickingPlugin);
    }
}

fn spawn_axis_marker(mut commands: Commands) {
    let bundle = (
        AxisMarker {
            length: 1.1,
            thickness: 0.2,
        },
        RenderLayers::layer(RENDER_LAYER),
    );
    commands.spawn(bundle);
}
