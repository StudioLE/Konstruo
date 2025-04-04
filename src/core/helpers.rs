use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

pub struct Helpers;

impl Helpers {
    /// Despawn entities with the specified parent.
    pub fn despawn_children<F: QueryFilter>(
        commands: &mut Commands,
        entities: &Query<(Entity, &ChildOf), F>,
        parent_entity: Entity,
    ) {
        for (entity, parent) in entities.iter() {
            if parent.parent != parent_entity {
                continue;
            }
            commands.entity(entity).despawn();
        }
    }
}
