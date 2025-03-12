use bevy::prelude::*;

/// Wireframe
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct Wireframe;
