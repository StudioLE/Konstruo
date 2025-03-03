use bevy::prelude::*;

#[derive(Component, Debug, Default, PartialEq)]
pub enum State {
    #[default]
    Enabled,
    Hovered,
    Selected,
}
