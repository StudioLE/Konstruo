use crate::infrastructure::{SplineChanged, Way};
use crate::ui::*;
use bevy::prelude::*;
use std::fmt::{Display, Formatter};
use Action::*;

#[derive(Clone, Copy, Debug, Event)]
pub enum Action {
    AddBuildings,
    AddWaySurface,
    Close,
    Edit,
    Deselect(Entity),
    DrawWay,
    FinishWay,
    Info,
    More,
    Remove(Entity),
    Settings,
    Undo,
}

impl Action {
    /// System to respond to [`Action`] events.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn event_system(
        mut commands: Commands,
        mut events: EventReader<Action>,
        mut interface: ResMut<InterfaceState>,
        mut changed: EventWriter<EntityStateChanged>,
        mut entity_states: Query<&mut EntityState>,
        mut drawing: ResMut<Drawing>,
        mut ways: Query<&mut Way>,
        mut spline_changed_event: EventWriter<SplineChanged>,
    ) {
        for event in events.read() {
            match event {
                Deselect(entity) => {
                    deselect(&mut changed, &mut entity_states, *entity);
                }
                FinishWay => {
                    drawing.on_complete(&mut ways, &mut spline_changed_event);
                }
                Remove(entity) => {
                    remove(&mut commands, *entity);
                }
                _ => {}
            };
            *interface = match event {
                DrawWay => InterfaceState::DrawWay,
                _ => InterfaceState::Default,
            };
        }
    }

    #[must_use]
    pub fn get_icon(&self) -> Icon {
        match self {
            AddBuildings => Icon::FontAwesome {
                name: String::from("home"),
            },
            AddWaySurface => Icon::FontAwesome {
                name: String::from("road"),
            },
            Close | Deselect(_) => Icon::FontAwesome {
                name: String::from("times"),
            },
            Edit => Icon::FontAwesome {
                name: String::from("edit"),
            },
            FinishWay => Icon::FontAwesome {
                name: String::from("check"),
            },
            DrawWay => Icon::FontAwesome {
                name: String::from("bezier-curve"),
            },
            Info => Icon::FontAwesome {
                name: String::from("info"),
            },
            More => Icon::FontAwesome {
                name: String::from("ellipsis-v-alt"),
            },
            Remove(_) => Icon::FontAwesome {
                name: String::from("trash"),
            },
            Settings => Icon::FontAwesome {
                name: String::from("cog"),
            },
            Undo => Icon::FontAwesome {
                name: String::from("undo"),
            },
        }
    }
}

impl Display for Action {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            AddBuildings => "Add Buildings".to_owned(),
            AddWaySurface => "Add Way Surface".to_owned(),
            Close => "Close".to_owned(),
            Deselect(_) => "Deselect".to_owned(),
            DrawWay => "Draw Way".to_owned(),
            FinishWay => "Finish Way".to_owned(),
            Edit => "Edit".to_owned(),
            Info => "Info".to_owned(),
            More => "More".to_owned(),
            Remove(_) => "Remove".to_owned(),
            Settings => "Settings".to_owned(),
            Undo => "Undo".to_owned(),
        };
        output.fmt(formatter)
    }
}

fn deselect(
    changed: &mut EventWriter<EntityStateChanged>,
    entity_states: &mut Query<&mut EntityState>,
    entity: Entity,
) {
    let Ok(mut state) = entity_states.get_mut(entity) else {
        warn!("Failed to get EntityState for {entity:?}");
        return;
    };
    *state = EntityState::Default;
    changed.send(EntityStateChanged {
        entity,
        state: EntityState::Default,
    });
}

fn remove(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).despawn_recursive();
}
