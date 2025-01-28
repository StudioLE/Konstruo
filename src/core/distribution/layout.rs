use crate::distribution::Distributable;
use bevy::math::Vec3;
use bevy::prelude::Transform;

/// Layout produced by the distribution algorithm.
pub struct Container {
    /// Size
    pub size: Vec3,
    /// Items
    pub items: Vec<DistributedItem>,
}

/// Layout item produced by the distribution algorithm.
pub struct DistributedItem {
    /// Item
    pub item: Box<dyn Distributable>,
    /// Size
    pub size: Vec3,
    /// Translation to the center relative to the center of the container
    pub translation: Vec3,
}
