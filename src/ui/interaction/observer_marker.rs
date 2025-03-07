use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;

/// Marker to identify the target of an [`Observer`] so it's retrievable.
#[derive(Component)]
pub struct ObserverMarker {
    pub target: Entity,
}

impl ObserverMarker {
    /// Create a new [`Observer`].
    #[must_use]
    pub fn new(target: Entity) -> Self {
        Self { target }
    }

    /// Spawn an [`Observer`] with [`ObserverMarker`].
    pub fn spawn<E: Event, B: Bundle, M, I: IntoObserverSystem<E, B, M>>(
        commands: &mut Commands,
        target: Entity,
        system: I,
    ) {
        commands.spawn((
            ObserverMarker::new(target),
            Observer::new(system).with_entity(target),
        ));
    }

    /// Despawn [`Observer`] with the specified target.
    pub fn despawn(
        commands: &mut Commands,
        observers: &Query<(Entity, &ObserverMarker), With<Observer>>,
        target: Entity,
    ) {
        for (observer, marker) in observers.iter() {
            if marker.target != target {
                continue;
            }
            commands.entity(observer).despawn();
        }
    }
}
