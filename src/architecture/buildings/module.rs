use crate::architecture::RoofStyle;
use crate::geometry::Vec6;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Component)]
#[require(InheritedVisibility, Transform)]
pub struct BuildingModule {
    /// Level number
    /// 0: Ground
    /// 1: First
    /// -1: Basement
    pub level: isize,
    /// Width from left to right
    pub width: f32,
    /// Length from front to back
    pub length: f32,
    /// Height from bottom to top
    pub height: f32,
    /// Margins or offsets
    pub margin: Option<Vec6>,
    /// Is this a roof module?
    pub roof: Option<RoofStyle>,
}
