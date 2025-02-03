use bevy::prelude::*;

/// Source item
#[derive(Clone, Debug, PartialEq)]
pub struct SourceItem {
    /// Size
    pub size: Vec3,
    /// Margin
    pub margin: Vec3,
}

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

impl Default for SourceItem {
    fn default() -> Self {
        Self {
            size: Vec3::ZERO,
            margin: Vec3::ZERO,
        }
    }
}
