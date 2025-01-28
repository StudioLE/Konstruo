use bevy::prelude::*;

/// Distributed item
pub struct Item {
    /// Original size
    pub original_size: Vec3,
    /// Size
    pub size: Vec3,
    /// Translation to the center relative to the center of the container
    pub translation: Vec3,
}
