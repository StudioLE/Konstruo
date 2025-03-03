use bevy::prelude::*;

#[derive(Component, Debug, Default, PartialEq)]
pub enum EntityState {
    #[default]
    Enabled,
    Hovered,
    Selected,
}
