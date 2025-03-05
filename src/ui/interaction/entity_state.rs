use bevy::prelude::*;

#[derive(Component, Debug, Default, PartialEq)]
pub enum EntityState {
    #[default]
    Default,
    Hovered,
    Selected,
}

#[derive(Debug, Event)]
pub struct EntityStateChanged {
    pub entity: Entity,
    pub state: EntityState,
}
