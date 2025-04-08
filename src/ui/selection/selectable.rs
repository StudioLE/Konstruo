use crate::extensions::*;
use crate::hierarchy::Ancestry;
use crate::ui::{EntityState, EntityStateChanged, InterfaceState};
use bevy::prelude::*;

#[derive(Component)]
pub struct Selectable {
    /// The number of levels above in the ancestry
    generation: usize,
}

impl Selectable {
    /// Create a new [`Selectable`].
    #[must_use]
    pub fn new(generation: usize) -> Self {
        Self { generation }
    }

    /// System to create [`Observer`] when [`Selectable`] is added.
    pub fn added_system(mut commands: Commands, entities: Query<Entity, Added<Selectable>>) {
        for entity in entities.iter() {
            commands.spawn(Observer::new(on_pointer_over).with_entity(entity));
            commands.spawn(Observer::new(on_pointer_out).with_entity(entity));
            commands.spawn(Observer::new(on_pointer_click).with_entity(entity));
        }
    }
}

fn get_ancestor_state<'a>(
    selectables: Query<&Selectable>,
    ancestors: Query<Option<&ChildOf>>,
    states: &'a mut Query<&mut EntityState>,
    entity: Entity,
) -> Option<(Entity, Mut<'a, EntityState>)> {
    let selectable = selectables.get(entity).handle_error(|e| {
        warn!("Failed to get Selectable for {entity}: {e}");
    })?;
    let ancestor =
        Ancestry::get_ancestor(&ancestors, entity, selectable.generation).handle_error(|e| {
            warn!("Failed to get ancestor for {entity}: {e}");
        })?;
    let state = states.get_mut(ancestor).handle_error(|e| {
        warn!("Failed to get EntityState for {ancestor}: {e}");
    })?;
    Some((ancestor, state))
}

fn on_pointer_over(
    trigger: Trigger<Pointer<Over>>,
    selectables: Query<&Selectable>,
    ancestors: Query<Option<&ChildOf>>,
    mut states: Query<&mut EntityState>,
    mut changed: EventWriter<EntityStateChanged>,
) {
    let Some((ancestor, mut state)) =
        get_ancestor_state(selectables, ancestors, &mut states, trigger.target())
    else {
        return;
    };
    if *state != EntityState::Selected {
        *state = EntityState::Hovered;
        changed.write(EntityStateChanged {
            entity: ancestor,
            state: EntityState::Hovered,
        });
    }
}

fn on_pointer_out(
    trigger: Trigger<Pointer<Out>>,
    selectables: Query<&Selectable>,
    ancestors: Query<Option<&ChildOf>>,
    mut states: Query<&mut EntityState>,
    mut changed: EventWriter<EntityStateChanged>,
) {
    let Some((ancestor, mut state)) =
        get_ancestor_state(selectables, ancestors, &mut states, trigger.target())
    else {
        return;
    };
    if *state != EntityState::Selected {
        *state = EntityState::Default;
        changed.write(EntityStateChanged {
            entity: ancestor,
            state: EntityState::Default,
        });
    }
}

fn on_pointer_click(
    trigger: Trigger<Pointer<Click>>,
    selectables: Query<&Selectable>,
    ancestors: Query<Option<&ChildOf>>,
    mut states: Query<&mut EntityState>,
    mut changed: EventWriter<EntityStateChanged>,
    mut interface: ResMut<InterfaceState>,
) {
    if trigger.button != PointerButton::Primary {
        return;
    }
    let Some((ancestor, mut state)) =
        get_ancestor_state(selectables, ancestors, &mut states, trigger.target())
    else {
        return;
    };
    if *state != EntityState::Selected {
        *state = EntityState::Selected;
        *interface = InterfaceState::PathSelected {
            path: ancestor,
            surface: trigger.target(),
        };
        changed.write(EntityStateChanged {
            entity: ancestor,
            state: EntityState::Selected,
        });
    }
}
