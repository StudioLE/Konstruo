use crate::*;
use bevy::prelude::*;
use konstruo_ui::{EntityState, EntityStateChanged};

#[derive(Debug, Resource, Default, PartialEq)]
pub enum InterfaceState {
    #[default]
    Default,
    DrawPath,
    /// An entity was selected.
    Selected {
        entity: Entity,
    },
}

impl InterfaceState {
    /// Get the [`Action`] for the current interface state.
    #[must_use]
    pub(super) fn get_actions(&self) -> Vec<Action> {
        match self {
            InterfaceState::Default => default_actions(),
            InterfaceState::DrawPath => Drawing::actions(),
            InterfaceState::Selected { .. } => Selection::actions(),
        }
    }

    /// Update [`InterfaceState`] on [`EntityStateChanged`].
    pub(super) fn on_entity_state_changed(
        mut events: EventReader<EntityStateChanged>,
        mut interface: ResMut<InterfaceState>,
    ) {
        for event in events.read() {
            if event.state == EntityState::Selected {
                *interface = InterfaceState::Selected {
                    entity: event.entity,
                }
            }
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

fn settings_action(_trigger: Trigger<Pointer<Released>>) {
    trace!("Settings button was pressed.");
    warn!("Settings was pressed but it is not implemented.");
}
