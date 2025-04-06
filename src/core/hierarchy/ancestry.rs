use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

pub struct Ancestry;

pub enum AncestorError {
    NotFound { entity: Entity, generation: usize },
}

impl Ancestry {
    /// Get all the ancestors of the specified entity.
    pub fn get_ancestors<F: QueryFilter>(
        entities: &Query<Option<&ChildOf>, F>,
        entity: Entity,
        max: Option<usize>,
    ) -> Result<Vec<Entity>, AncestorError> {
        let mut ancestors = Vec::new();
        let mut current = entity;
        let mut generation = 0;
        loop {
            let Ok(child_of) = entities.get(current) else {
                return Err(AncestorError::NotFound { entity, generation });
            };
            let Some(child_of) = child_of else {
                break;
            };
            ancestors.push(child_of.parent);
            current = child_of.parent;
            generation += 1;
            if let Some(max) = max {
                if generation > max {
                    break;
                }
            }
        }
        Ok(ancestors)
    }

    /// Get a specific ancestor of the specified entity.
    #[must_use]
    pub fn get_ancestor<F: QueryFilter>(
        entities: &Query<Option<&ChildOf>, F>,
        entity: Entity,
        generation: usize,
    ) -> Option<Entity> {
        Ancestry::get_ancestors(entities, entity, Some(generation))
            .ok()?
            .get(generation - 1)
            .copied()
    }
}
