use crate::ui::*;
use bevy::log::warn;
use bevy::prelude::*;

pub struct Selection;

impl Selection {
    /// Get the actions when [`Selection`] is active.
    pub(crate) fn actions() -> Vec<Action> {
        vec![
            Action {
                label: String::from("Deselect"),
                icon: Icon::font_awesome("times"),
                on_press: Observer::new(Selection::deselect_action),
            },
            Action {
                label: String::from("Remove"),
                icon: Icon::font_awesome("trash"),
                on_press: Observer::new(Selection::remove_action),
            },
            Action {
                label: String::from("Info"),
                icon: Icon::font_awesome("info"),
                on_press: Observer::new(Selection::info_action),
            },
            Action {
                label: String::from("Add Surface"),
                icon: Icon::font_awesome("road"),
                on_press: Observer::new(Selection::add_surface_action),
            },
            Action {
                label: String::from("Add Buildings"),
                icon: Icon::font_awesome("home"),
                on_press: Observer::new(Selection::add_buildings_action),
            },
        ]
    }

    fn add_buildings_action(_trigger: Trigger<Pointer<Up>>) {
        trace!("Add buildings button was pressed.");
        warn!("Add buildings action not implemented");
    }

    fn add_surface_action(_trigger: Trigger<Pointer<Up>>) {
        trace!("Add surface button was pressed.");
        warn!("Add surface action not implemented");
    }

    /// Deselect the selected entity on action button press.
    fn deselect_action(
        _trigger: Trigger<Pointer<Up>>,
        mut changed: EventWriter<EntityStateChanged>,
        mut entity_states: Query<&mut EntityState>,
        mut interface: ResMut<InterfaceState>,
    ) {
        trace!("Deselect button was pressed.");
        let InterfaceState::PathSelected { path, .. } = *interface else {
            warn!("Expected InterfaceState::PathSelected: {interface:?}");
            return;
        };
        let Ok(mut entity_state) = entity_states.get_mut(path) else {
            warn!("Failed to get EntityState for {path:?}");
            return;
        };
        *entity_state = EntityState::Default;
        changed.send(EntityStateChanged {
            entity: path,
            state: EntityState::Default,
        });
        *interface = InterfaceState::Default;
    }

    fn info_action(_trigger: Trigger<Pointer<Up>>) {
        trace!("Info button was pressed.");
        warn!("Info action not implemented");
    }

    /// Remove the selected entity on action button press
    fn remove_action(
        _trigger: Trigger<Pointer<Up>>,
        mut commands: Commands,
        mut interface: ResMut<InterfaceState>,
    ) {
        trace!("Remove button was pressed.");
        let InterfaceState::PathSelected { path, .. } = *interface else {
            warn!("Expected InterfaceState::PathSelected: {interface:?}");
            return;
        };
        commands.entity(path).despawn_recursive();
        *interface = InterfaceState::Default;
    }
}
