use crate::ui::*;
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

impl InterfaceState {
    /// Get the [`Action`] for the current interface state.
    #[must_use]
    pub(super) fn get_actions(&self) -> Vec<Action> {
        match self {
            InterfaceState::Default => default_actions(),
            InterfaceState::DrawWay => Drawing::actions(),
            InterfaceState::WaySelected { .. } => Selection::actions(),
        }
    }
}

fn default_actions() -> Vec<Action> {
    vec![
        Action {
            label: String::from("Settings"),
            icon: Icon::font_awesome("cog"),
            on_press: Observer::new(settings_action),
        },
        Action {
            label: String::from("Draw Way"),
            icon: Icon::font_awesome("bezier-curve"),
            on_press: Observer::new(Drawing::start_action),
        },
    ]
}

fn settings_action(_trigger: Trigger<Pointer<Up>>) {
    trace!("Settings button was pressed.");
    warn!("Settings was pressed but it is not implemented.");
}
