use crate::distribution::Distributed;
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
    /// Default is None
    pub size: Option<Vec3>,
    /// Margins around this item.
    ///
    /// Margins will collapse between items but not at edges.
    ///
    /// Default is None
    pub margin: Option<Vec6>,
}

impl Default for Distributable {
    fn default() -> Self {
        Self {
            order: usize::MAX,
            size: None,
            margin: None,
        }
    }
}

impl Distributable {
    #[must_use]
    pub fn to_distributed(self: Distributable) -> Distributed {
        Distributed {
            size: Vec3::ZERO,
            translation: Vec3::ZERO,
            source: self,
        }
    }
}
