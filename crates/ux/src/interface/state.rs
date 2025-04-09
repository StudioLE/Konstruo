use crate::*;
use bevy::prelude::*;
use konstruo_architecture::ModularBuilding;
use konstruo_core::HandleError;
use konstruo_paths::Path;
use konstruo_ui::{EntityState, EntityStateChanged};

#[derive(Debug, Resource, Default, PartialEq)]
pub enum InterfaceState {
    #[default]
    Default,
    DrawPath,
    Selection(SelectionMode, Entity),
}

impl InterfaceState {
    /// Get the [`Action`] for the current interface state.
    #[must_use]
    pub(super) fn get_actions(&self) -> Vec<Action> {
        match self {
            InterfaceState::Default => default_actions(),
            InterfaceState::DrawPath => DrawMode::actions(),
            InterfaceState::Selection(mode, _) => mode.actions(),
        }
    }

    /// Update [`InterfaceState`] on [`EntityStateChanged`].
    pub(crate) fn on_entity_state_changed(
        mut events: EventReader<EntityStateChanged>,
        mut interface: ResMut<InterfaceState>,
        query: Query<(Option<&ModularBuilding>, Option<&Path>)>,
    ) {
        for event in events.read() {
            if event.state == EntityState::Selected {
                let Some((building, path)) = query
                    .get(event.entity)
                    .handle_error(|e| warn!("Failed to get entity: {e}"))
                else {
                    continue;
                };
                let mode = if building.is_some() {
                    SelectionMode::Building
                } else if path.is_some() {
                    SelectionMode::Path
                } else {
                    SelectionMode::Default
                };
                *interface = InterfaceState::Selection(mode, event.entity);
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
            on_press: Observer::new(DrawMode::start_action),
        },
    ]
}

fn settings_action(trigger: Trigger<Pointer<Released>>) {
    if trigger.button != PointerButton::Primary {
        return;
    }
    trace!("Settings button was pressed.");
    warn!("Settings was pressed but it is not implemented.");
}
