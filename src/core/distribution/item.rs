use super::*;
use bevy::prelude::*;

/// Distributed item
#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    /// Size
    pub size: Vec3,
    /// Translation to the center relative to the center of the container
    pub translation: Vec3,
    /// Source item
    pub source: SourceItem,
}

impl From<SourceItem> for Item {
    fn from(source: SourceItem) -> Self {
        Item {
            size: source.size,
            translation: Vec3::ZERO,
            source,
        }
    }
}
