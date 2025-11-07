use crate::*;
use bevy::prelude::*;
use konstruo_core::*;
use std::collections::HashSet;

/// Set the [`Visibility`] according to the [`EntityState`] of an ancestor.
#[derive(Component)]
pub struct OnEntityState {
    /// The number of levels above in the ancestry
    generation: usize,
    condition: Vec<EntityState>,
}

impl OnEntityState {
    /// Create a new [`OnEntityState`].
    #[must_use]
    pub fn new(generation: usize, condition: Vec<EntityState>) -> Self {
        Self {
            generation,
            condition,
        }
    }

    /// Update the visibility when the [`EntityState`] of an ancestor changes.
    pub(super) fn on_state_changed(
        mut events: MessageReader<EntityStateChanged>,
        filter: Query<(Entity, &OnEntityState)>,
        ancestors: Query<Option<&ChildOf>>,
        mut visibilities: Query<&mut Visibility>,
    ) {
        let mut duplicates = 0;
        let mut updated = HashSet::new();
        for event in events.read() {
            if !updated.insert(event) {
                duplicates += 1;
                continue;
            }
            for (entity, on) in filter {
                let Some(ancestor) =
                    entity
                        .get_ancestor(&ancestors, on.generation)
                        .handle_error(|e| {
                            warn!("Failed to get ancestor for {entity}: {e}");
                        })
                else {
                    continue;
                };
                if ancestor != event.entity {
                    continue;
                }
                let Some(mut visibility) = visibilities.get_mut(entity).handle_error(|e| {
                    warn!("Failed to get Visibility for {entity}: {e}");
                }) else {
                    continue;
                };
                *visibility = if on.condition.contains(&event.state) {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
        if duplicates > 0 {
            trace!("Ignored {duplicates} duplicate EntityStateChanged events");
        }
    }
}
