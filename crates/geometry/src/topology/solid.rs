use bevy::prelude::*;

/// A representation of solid geometry of an entity.
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct Solid;
