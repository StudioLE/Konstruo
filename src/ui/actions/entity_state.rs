use bevy::prelude::*;

#[derive(Component, Debug, Default, PartialEq)]
pub enum EntityState {
    #[default]
    Default,
    Hovered,
    Selected,
}
