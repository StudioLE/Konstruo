use bevy::prelude::*;

#[derive(Debug, Resource, Default, PartialEq)]
pub enum InterfaceState {
    #[default]
    Default,
    DrawWay,
    /// A [`Way`] was selected by clicking on a [`WaySurface`].
    WaySelected {
        /// [`Way`]
        way: Entity,
        /// [`WaySurface`] that was selected
        surface: Entity,
    },
}
