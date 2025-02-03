use crate::geometry::Vec6;
use bevy::prelude::*;

/// Source item
#[derive(Clone, Debug, PartialEq)]
pub struct SourceItem {
    /// Size
    pub size: Vec3,
    /// Margin
    pub margin: Vec6,
}

impl Default for SourceItem {
    fn default() -> Self {
        Self {
            size: Vec3::ZERO,
            margin: Vec6::ZERO,
        }
    }
}
