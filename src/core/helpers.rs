use bevy::ecs::query::QueryFilter;
use bevy::hierarchy::Parent;
use bevy::prelude::{Commands, Entity, Query};

pub struct Helpers;

impl Helpers {
    /// Despawn entities with the specified parent.
    pub fn despawn_children<F: QueryFilter>(
        commands: &mut Commands,
        entities: &Query<(Entity, &Parent), F>,
        parent_entity: Entity,
    ) {
        for (entity, parent) in entities.iter() {
            if parent.get() != parent_entity {
                continue;
            }
            commands.entity(entity).despawn();
        }
    }
}
