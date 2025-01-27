use crate::architecture::RoofStyle;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Component)]
#[require(InheritedVisibility, Transform)]
pub struct BuildingModule {
    /// Index of the module within the building storey
    pub index: usize,
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
    /// Offset at front
    pub front_offset: f32,
    /// Offset at Back
    pub back_offset: f32,
    /// Is this a roof module?
    pub roof: Option<RoofStyle>,
}
