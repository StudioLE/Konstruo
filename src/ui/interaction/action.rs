use crate::ui::*;
use bevy::prelude::*;
use std::fmt::{Display, Formatter};
use Action::*;

#[derive(Clone, Copy, Debug, Event)]
pub enum Action {
    AddBuildings,
    AddWaySurface,
    Close,
    Done,
    Edit,
    Deselect(Entity),
    DrawWay,
    Info,
    More,
    Remove(Entity),
    Settings,
    Undo,
}

impl Action {
    /// System to respond to [`Action`] events.
    pub(super) fn event_system(
        mut commands: Commands,
        mut events: EventReader<Action>,
        mut interface: EventWriter<InterfaceState>,
        mut changed: EventWriter<EntityStateChanged>,
        mut entity_states: Query<&mut EntityState>,
    ) {
        for event in events.read() {
            match event {
                Close | Done => done(&mut interface),
                Deselect(entity) => {
                    deselect(&mut interface, &mut changed, &mut entity_states, *entity);
                }
                DrawWay => draw_way(&mut commands, &mut interface),
                Remove(entity) => remove(&mut commands, &mut interface, *entity),
                _ => {
                    warn!("Unhandled Action: {event:?}");
                }
            }
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
            Done => Icon::FontAwesome {
                name: String::from("check"),
            },
            Edit => Icon::FontAwesome {
                name: String::from("edit"),
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
            Done => "Done".to_owned(),
            Edit => "Edit".to_owned(),
            Deselect(_) => "Deselect".to_owned(),
            DrawWay => "Draw Way".to_owned(),
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
    interface: &mut EventWriter<InterfaceState>,
    changed: &mut EventWriter<EntityStateChanged>,
    entity_states: &mut Query<&mut EntityState>,
    entity: Entity,
) {
    interface.send(InterfaceState::Default);
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

fn done(interface: &mut EventWriter<InterfaceState>) {
    interface.send(InterfaceState::Default);
}

fn draw_way(_commands: &mut Commands, interface: &mut EventWriter<InterfaceState>) {
    interface.send(InterfaceState::DrawWay);
}

fn remove(commands: &mut Commands, interface: &mut EventWriter<InterfaceState>, entity: Entity) {
    interface.send(InterfaceState::Default);
    commands.entity(entity).despawn_recursive();
}
