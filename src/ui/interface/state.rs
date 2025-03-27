use crate::ui::*;
use bevy::prelude::*;

#[derive(Debug, Resource, Default, PartialEq)]
pub enum InterfaceState {
    #[default]
    Default,
    DrawPath,
    /// A [`Path`] was selected by clicking on a [`PathSurface`].
    PathSelected {
        /// [`Path`]
        path: Entity,
        /// [`PathSurface`] that was selected
        surface: Entity,
    },
}

impl InterfaceState {
    /// Get the [`Action`] for the current interface state.
    #[must_use]
    pub(super) fn get_actions(&self) -> Vec<Action> {
        match self {
            InterfaceState::Default => default_actions(),
            InterfaceState::DrawPath => Drawing::actions(),
            InterfaceState::PathSelected { .. } => Selection::actions(),
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
            label: String::from("Draw Path"),
            icon: Icon::font_awesome("bezier-curve"),
            on_press: Observer::new(Drawing::start_action),
        },
    ]
}

fn settings_action(_trigger: Trigger<Pointer<Up>>) {
    trace!("Settings button was pressed.");
    warn!("Settings was pressed but it is not implemented.");
}
