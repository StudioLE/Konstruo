use crate::{EntityState, EntityStateChanged};
use bevy::prelude::*;
use konstruo_core::{AncestryExtensions, EntityExtensions, HandleError};

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
    states: &'a mut Query<(Entity, &mut EntityState)>,
    entity: Entity,
) -> Option<(Entity, Mut<'a, EntityState>)> {
    let selectable = selectables.get(entity).handle_error(|e| {
        warn!("Failed to get Selectable for {entity}: {e}");
    })?;
    let ancestor = entity
        .get_ancestor(&ancestors, selectable.generation)
        .handle_error(|e| {
            warn!("Failed to get ancestor for {entity}: {e}");
        })?;
    states.get_mut(ancestor).handle_error(|e| {
        warn!("Failed to get EntityState for {ancestor}: {e}");
    })
}

fn on_pointer_over(
    trigger: Trigger<Pointer<Over>>,
    selectables: Query<&Selectable>,
    ancestors: Query<Option<&ChildOf>>,
    mut states: Query<(Entity, &mut EntityState)>,
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
    mut states: Query<(Entity, &mut EntityState)>,
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
    mut states: Query<(Entity, &mut EntityState)>,
    mut changed: EventWriter<EntityStateChanged>,
    names: Query<&Name>,
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
        trace!("Selected `{}`", ancestor.id_with_name(&names));
        changed.write(EntityStateChanged {
            entity: ancestor,
            state: EntityState::Selected,
        });
        for (entity, mut state) in &mut states {
            if entity == ancestor {
                continue;
            }
            if *state == EntityState::Selected {
                trace!("De-selected `{}`", entity.id_with_name(&names));
                *state = EntityState::Default;
                changed.write(EntityStateChanged {
                    entity,
                    state: EntityState::Default,
                });
            }
        }
    }
}
