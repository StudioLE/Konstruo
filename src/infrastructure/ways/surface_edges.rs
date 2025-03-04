use super::*;
use crate::ui::EntityState;
use bevy::prelude::*;

/// An edge of a [`WaySurface`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WaySurfaceEdge {
    pub way: Entity,
    pub index: usize,
}

impl WaySurfaceEdge {
    /// Update the [`WaySurfaceEdges`] visibility when the [`EntityState`] of the [`Way`] changes.
    pub(super) fn on_state_changed(
        mut events: EventReader<StateChangedEvent>,
        mut edges: Query<(&WaySurfaceEdge, &mut Visibility)>,
    ) {
        for event in events.read() {
            for (edge, mut visibility) in &mut edges {
                if edge.way != event.way {
                    continue;
                }
                *visibility = match event.state {
                    EntityState::Default => Visibility::Hidden,
                    EntityState::Hovered | EntityState::Selected => Visibility::Visible,
                };
            }
        }
    }
}
