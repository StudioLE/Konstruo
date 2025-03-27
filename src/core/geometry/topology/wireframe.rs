use bevy::prelude::*;

/// Wireframe of a mesh.
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct Wireframe;
