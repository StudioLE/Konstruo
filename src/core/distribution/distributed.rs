use super::*;
use bevy::prelude::*;

/// Distributed item
#[derive(Clone, Debug, PartialEq)]
pub struct Distributed {
    /// Cuboid bounds
    ///
    /// This may be different to the source size due to flex grow or flex shrink.
    pub size: Vec3,
    /// Translation to the center relative to the center of the container
    pub translation: Vec3,
    /// Source item
    pub source: Distributable,
}

impl From<Distributable> for Distributed {
    fn from(source: Distributable) -> Self {
        Distributed {
            size: Vec3::ZERO,
            translation: Vec3::ZERO,
            source,
        }
    }
}
