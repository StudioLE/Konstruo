use crate::ui::*;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use Action::*;
use FloatingActionButtonSize::{Medium, Small};

#[derive(Debug, Event, Default, PartialEq)]
pub enum InterfaceState {
    #[default]
    Default,
    DrawWay,
    /// A [`Way`] was selected by clicking on a [`WaySurface`].
    WaySelected {
        /// [`Way`]
        way: Entity,
        /// [`WaySurface`] that was selected
        surface: Entity,
    },
}

impl InterfaceState {
    /// System to update the [`ActionsBar`] when [`InterfaceState`] is triggered.
    pub(super) fn event_system(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut events: EventReader<InterfaceState>,
        buttons: Query<Entity, With<FloatingActionButton>>,
        bars: Query<Entity, (With<ActionsBar>, Without<InterfaceState>)>,
    ) {
        for event in events.read() {
            trace!("InterfaceEvent triggered: {event:?}");
            let Ok(bar) = bars.get_single() else {
                warn!("Failed to get ActionsBar");
                return;
            };
            for entity in buttons.iter() {
                commands.entity(entity).despawn_recursive();
            }
            event.spawn_actions(&mut commands, &assets, bar);
        }
    }

    #[must_use]
    pub fn get_actions(&self) -> Vec<Action> {
        match self {
            InterfaceState::Default => {
                vec![Settings, DrawWay]
            }
            InterfaceState::DrawWay => {
                vec![Close, Undo, Done]
            }
            InterfaceState::WaySelected { way, .. } => {
                vec![
                    Deselect(*way),
                    Remove(*way),
                    Info,
                    AddWaySurface,
                    AddBuildings,
                ]
            }
        }
    }

    pub fn spawn_actions(&self, commands: &mut Commands, assets: &Res<AssetServer>, bar: Entity) {
        let actions = self.get_actions();
        let last = actions.len() - 1;
        for (i, action) in actions.into_iter().enumerate() {
            let size = if i == last { Medium } else { Small };
            let icon = action.get_icon().get_path();
            FloatingActionButton::spawn(commands, action, size, assets.load(icon), bar);
        }
    }
}
