use bevy::prelude::*;

/// Selection state of an entity.
#[derive(Component, Debug, Default, Eq, Hash, PartialEq)]
pub enum EntityState {
    #[default]
    Default,
    Hovered,
    Selected,
}

/// An event indicating the state of an entity has changed.
#[derive(Debug, Event, Eq, Hash, PartialEq)]
pub struct EntityStateChanged {
    pub entity: Entity,
    pub state: EntityState,
}
