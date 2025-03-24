use crate::ui::*;
use bevy::prelude::*;

pub struct Action {
    pub label: String,
    pub icon: Icon,
    pub on_press: Observer,
}
