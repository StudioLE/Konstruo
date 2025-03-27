use bevy::prelude::*;

/// Edge of a mesh.
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct Edge;
