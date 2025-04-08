use bevy::prelude::*;

/// A representation of the edges of an entity.
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct Edge;
