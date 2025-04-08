use bevy::ecs::query::{QueryEntityError, QueryFilter};
use bevy::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AncestorError {
    Query(QueryEntityError),
    Count { expected: usize, actual: usize },
}

pub trait AncestryExtensions {
    fn get_ancestors<F: QueryFilter>(
        self,
        entities: &Query<Option<&ChildOf>, F>,
        max: Option<usize>,
    ) -> Result<Vec<Entity>, QueryEntityError>;
    fn get_ancestor<F: QueryFilter>(
        self,
        entities: &Query<Option<&ChildOf>, F>,
        generation: usize,
    ) -> Result<Entity, AncestorError>;
}

impl AncestryExtensions for Entity {
    /// Get all the ancestors of an [`Entity`].
    ///
    /// An optional `max` number of generations can be
    ///
    /// A [`QueryFilter`] may be applied but is not recommended.
    ///
    /// May return a [`QueryEntityError`] if [`Query`] does not contain an ancestor.
    fn get_ancestors<F: QueryFilter>(
        self,
        entities: &Query<Option<&ChildOf>, F>,
        max: Option<usize>,
    ) -> Result<Vec<Entity>, QueryEntityError> {
        let mut ancestors = Vec::new();
        let mut current = self;
        let mut generation = 0;
        loop {
            let Some(child_of) = entities.get(current)? else {
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
    #[allow(clippy::indexing_slicing)]
    fn get_ancestor<F: QueryFilter>(
        self,
        entities: &Query<Option<&ChildOf>, F>,
        generation: usize,
    ) -> Result<Entity, AncestorError> {
        let ancestors = self
            .get_ancestors(entities, Some(generation))
            .map_err(AncestorError::Query)?;
        if ancestors.len() < generation {
            Err(AncestorError::Count {
                expected: generation,
                actual: ancestors.len(),
            })
        } else {
            Ok(ancestors[generation - 1])
        }
    }
}

impl Display for AncestorError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let reason = match self {
            AncestorError::Query(e) => e.to_string(),
            AncestorError::Count { expected, actual } => {
                format!("Expected {expected} ancestors but only found {actual}",)
            }
        };
        formatter.write_str(&reason)
    }
}
