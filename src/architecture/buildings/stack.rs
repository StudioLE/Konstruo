use super::*;
use bevy::prelude::*;

#[derive(Clone, Component, Debug)]
#[require(InheritedVisibility, Transform)]
pub struct BuildingModuleStack {
    pub definition: BuildingModule,
    pub levels: usize,
    pub level_height: f32,
    pub roof_height: f32,
    pub roof_style: Option<RoofStyle>,
}
