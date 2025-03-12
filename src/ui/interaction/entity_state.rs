use bevy::prelude::*;

#[derive(Component, Debug, Default, Eq, Hash, PartialEq)]
pub enum EntityState {
    #[default]
    Default,
    Hovered,
    Selected,
}

#[derive(Debug, Event, Eq, Hash, PartialEq)]
pub struct EntityStateChanged {
    pub entity: Entity,
    pub state: EntityState,
}
