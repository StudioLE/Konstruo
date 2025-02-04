use super::*;
use bevy::prelude::*;

/// Container of distributed items
#[derive(Clone, Debug, PartialEq)]
pub struct Container {
    /// Size
    pub size: Vec3,
    /// Items
    pub items: Vec<Distributed>,
}
