use bevy::prelude::*;

/// Selection state of an entity.
#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EntityState {
    #[default]
    Default,
    Hovered,
    Selected,
}

/// An event indicating the state of an entity has changed.
#[derive(Debug, Eq, Hash, Message, PartialEq)]
pub struct EntityStateChanged {
    pub entity: Entity,
    pub state: EntityState,
}
