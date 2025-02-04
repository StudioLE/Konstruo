use crate::geometry::Vec6;
use bevy::prelude::*;

/// Properties that define how the entity will be distributed.
#[derive(Clone, Component, Debug, PartialEq)]
#[require(InheritedVisibility, Transform)]
pub struct Distributable {
    /// Order in which the item is distributed.
    ///
    /// Lowest will be distributed first.
    ///
    /// Default is last.
    pub order: usize,
    /// Cuboid bounds
    ///
    /// Default is zero
    pub size: Vec3,
    /// Margins around this item.
    ///
    /// Margins will collapse between items but not at edges.
    ///
    /// Default is zero
    pub margin: Vec6,
}

impl Default for Distributable {
    fn default() -> Self {
        Self {
            order: usize::MAX,
            size: Vec3::ZERO,
            margin: Vec6::ZERO,
        }
    }
}
