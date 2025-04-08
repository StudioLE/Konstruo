use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

pub trait EntityExtensions {
    fn despawn_children<F: QueryFilter>(
        self,
        commands: &mut Commands,
        entities: &Query<(Entity, &ChildOf), F>,
    );
}

impl EntityExtensions for Entity {
    /// Despawn entities with the specified parent.
    fn despawn_children<F: QueryFilter>(
        self,
        commands: &mut Commands,
        entities: &Query<(Entity, &ChildOf), F>,
    ) {
        for (entity, child_of) in entities.iter() {
            if child_of.parent != self {
                continue;
            }
            commands.entity(entity).despawn();
        }
    }
}
