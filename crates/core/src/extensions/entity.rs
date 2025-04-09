use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

pub trait EntityExtensions {
    fn id_with_name<F: QueryFilter>(self, names: &Query<&Name, F>) -> String;
    fn despawn_children<F: QueryFilter>(
        self,
        commands: &mut Commands,
        entities: &Query<(Entity, &ChildOf), F>,
    );
}

impl EntityExtensions for Entity {
    /// Get the id of the [`Entity`].
    ///
    /// Include [`Name`] if it exists.
    #[must_use]
    fn id_with_name<F: QueryFilter>(self, names: &Query<&Name, F>) -> String {
        let mut output = String::new();
        if let Ok(name) = names.get(self) {
            output.push_str(name.as_str());
            output.push(' ');
        }
        output.push_str(self.to_string().as_str());
        output
    }

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
