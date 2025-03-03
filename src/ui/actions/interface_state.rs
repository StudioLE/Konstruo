use bevy::prelude::*;

#[derive(Component, Debug, Default, PartialEq)]
pub enum InterfaceState {
    #[default]
    Default,
    WaySelected {
        way: Entity,
    },
}
use crate::ui::*;
use bevy::asset::AssetServer;
use FloatingActionButtonSize::{Medium, Small};

impl InterfaceState {
    #[must_use]
    pub fn get_actions(&self) -> Vec<Action> {
        match self {
            InterfaceState::Default => {
                vec![Action::settings(), Action::draw_way()]
            }
            InterfaceState::WaySelected { .. } => {
                vec![
                    Action::deselect(),
                    Action::remove(),
                    Action::info(),
                    Action::add_way_surface(),
                    Action::add_buildings(),
                ]
            }
        }
    }

    pub fn spawn_actions(&self, commands: &mut Commands, assets: &Res<AssetServer>, bar: Entity) {
        let actions = self.get_actions();
        let last = actions.len() - 1;
        for (i, action) in actions.into_iter().enumerate() {
            let size = if i == last { Medium } else { Small };
            action.spawn_fab(commands, assets, size, bar);
        }
    }
}
