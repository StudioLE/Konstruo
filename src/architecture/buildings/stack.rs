use super::*;
use bevy::prelude::*;

/// A vertical stack of [`BuildingModule`].
#[derive(Clone, Component, Debug)]
#[require(InheritedVisibility, Transform)]
pub struct BuildingModuleStack {
    pub modules: Vec<BuildingModule>,
}
