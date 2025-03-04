use crate::ui::*;
use bevy::prelude::*;
use std::fmt::{Display, Formatter};
use Action::*;

#[derive(Clone, Copy, Debug, Event)]
pub enum Action {
    AddBuildings,
    AddWaySurface,
    Edit,
    Deselect { way: Entity },
    DrawWay,
    Info,
    More,
    Remove,
    Settings,
}

impl Action {
    /// System to respond to [`Action`] events.
    pub(super) fn event_system(
        mut entity_states: Query<&mut EntityState>,
        mut events: EventReader<Action>,
        mut interface: EventWriter<InterfaceState>,
    ) {
        for event in events.read() {
            trace!("Action triggered: {event:?}");
            match event {
                Deselect { way } => {
                    interface.send(InterfaceState::Default);
                    let Ok(mut state) = entity_states.get_mut(*way) else {
                        warn!("Failed to get EntityState for {way:?}");
                        return;
                    };
                    *state = EntityState::Default;
                }
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
            Edit => Icon::FontAwesome {
                name: String::from("edit"),
            },
            Deselect { .. } => Icon::FontAwesome {
                name: String::from("times"),
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
            Remove => Icon::FontAwesome {
                name: String::from("trash"),
            },
            Settings => Icon::FontAwesome {
                name: String::from("cog"),
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
            Edit => "Edit".to_owned(),
            Deselect { .. } => "Deselect".to_owned(),
            DrawWay => "Draw Way".to_owned(),
            Info => "Info".to_owned(),
            More => "More".to_owned(),
            Remove => "Remove".to_owned(),
            Settings => "Settings".to_owned(),
        };
        output.fmt(formatter)
    }
}
