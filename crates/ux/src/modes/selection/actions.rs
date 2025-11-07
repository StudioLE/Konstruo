use crate::*;
use bevy::log::warn;
use bevy::prelude::*;
use konstruo_ui::{EntityState, EntityStateChanged};

impl SelectionMode {
    /// Get the actions when [`SelectionMode`] is active.
    pub(crate) fn actions(self) -> Vec<Action> {
        let mut actions = vec![
            Action {
                label: String::from("Deselect"),
                icon: Icon::font_awesome("times"),
                on_press: Observer::new(SelectionMode::deselect_action),
            },
            Action {
                label: String::from("Remove"),
                icon: Icon::font_awesome("trash"),
                on_press: Observer::new(SelectionMode::remove_action),
            },
            Action {
                label: String::from("Info"),
                icon: Icon::font_awesome("info"),
                on_press: Observer::new(SelectionMode::info_action),
            },
        ];
        if self == SelectionMode::Path {
            actions.push(Action {
                label: String::from("Add Surface"),
                icon: Icon::font_awesome("road"),
                on_press: Observer::new(SelectionMode::add_surface_action),
            });
            actions.push(Action {
                label: String::from("Add Buildings"),
                icon: Icon::font_awesome("home"),
                on_press: Observer::new(SelectionMode::add_buildings_action),
            });
        }
        actions
    }

    fn add_buildings_action(trigger: On<Pointer<Release>>) {
        if trigger.button != PointerButton::Primary {
            return;
        }
        trace!("Add buildings button was pressed.");
        warn!("Add buildings action not implemented");
    }

    fn add_surface_action(trigger: On<Pointer<Release>>) {
        if trigger.button != PointerButton::Primary {
            return;
        }
        trace!("Add surface button was pressed.");
        warn!("Add surface action not implemented");
    }

    /// Deselect the selected entity on action button press.
    fn deselect_action(
        trigger: On<Pointer<Release>>,
        mut changed: MessageWriter<EntityStateChanged>,
        mut entity_states: Query<&mut EntityState>,
        mut interface: ResMut<InterfaceState>,
    ) {
        if trigger.button != PointerButton::Primary {
            return;
        }
        trace!("Deselect button was pressed.");
        let InterfaceState::Selection(_, entity) = *interface else {
            warn!("Expected InterfaceState::Selection: {interface:?}");
            return;
        };
        let Ok(mut entity_state) = entity_states.get_mut(entity) else {
            warn!("Failed to get EntityState for {entity:?}");
            return;
        };
        *entity_state = EntityState::Default;
        changed.write(EntityStateChanged {
            entity,
            state: EntityState::Default,
        });
        *interface = InterfaceState::Default;
    }

    fn info_action(trigger: On<Pointer<Release>>) {
        if trigger.button != PointerButton::Primary {
            return;
        }
        trace!("Info button was pressed.");
        warn!("Info action not implemented");
    }

    /// Remove the selected entity on action button press
    fn remove_action(
        trigger: On<Pointer<Release>>,
        mut commands: Commands,
        mut interface: ResMut<InterfaceState>,
    ) {
        if trigger.button != PointerButton::Primary {
            return;
        }
        trace!("Remove button was pressed.");
        let InterfaceState::Selection(_, entity) = *interface else {
            warn!("Expected InterfaceState::Selection: {interface:?}");
            return;
        };
        commands.entity(entity).despawn();
        *interface = InterfaceState::Default;
    }
}
