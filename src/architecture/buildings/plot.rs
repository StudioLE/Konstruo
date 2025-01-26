use bevy::prelude::*;

#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct BuildingPlot {
    /// Width from left to right
    pub width: f32,
    /// Length from front to back
    pub length: f32,
    /// Height from bottom to top
    pub height: f32,
}
