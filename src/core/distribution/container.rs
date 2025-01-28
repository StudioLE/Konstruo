use super::*;
use bevy::prelude::*;

/// Container of distributed items
pub struct Container {
    /// Size
    pub size: Vec3,
    /// Items
    pub items: Vec<Item>,
}
