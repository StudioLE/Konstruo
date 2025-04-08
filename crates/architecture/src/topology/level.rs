use bevy::prelude::Component;

/// Building level or storey.
#[derive(Clone, Component, Debug, Default)]
pub struct Level {
    /// Level number
    /// 0: Ground
    /// 1: First
    /// -1: Basement
    pub level: isize,
}
