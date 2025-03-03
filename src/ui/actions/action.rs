use crate::ui::{FloatingActionButton, FloatingActionButtonSize};
use bevy::prelude::*;

pub struct Action {
    pub name: String,
    pub icon_category: String,
    pub icon_name: String,
}

impl Action {
    #[must_use]
    pub fn deselect() -> Self {
        Self {
            name: String::from("Deselect"),
            icon_category: String::from("navigation"),
            icon_name: String::from("close"),
        }
    }

    #[must_use]
    pub fn draw_way() -> Self {
        Self {
            name: String::from("Draw Way"),
            icon_category: String::from("content"),
            icon_name: String::from("gesture"),
        }
    }

    #[must_use]
    pub fn more() -> Self {
        Self {
            name: String::from("Draw Way"),
            icon_category: String::from("navigation"),
            icon_name: String::from("draw"),
        }
    }

    #[must_use]
    pub fn settings() -> Self {
        Self {
            name: String::from("Settings"),
            icon_category: String::from("action"),
            icon_name: String::from("settings"),
        }
    }

    #[must_use]
    pub fn edit() -> Self {
        Self {
            name: String::from("Edit"),
            icon_category: String::from("image"),
            icon_name: String::from("edit"),
        }
    }

    #[must_use]
    pub fn remove() -> Self {
        Self {
            name: String::from("Remove"),
            icon_category: String::from("action"),
            icon_name: String::from("delete"),
        }
    }

    #[must_use]
    pub fn info() -> Self {
        Self {
            name: String::from("Remove"),
            icon_category: String::from("action"),
            icon_name: String::from("info"),
        }
    }

    #[must_use]
    pub fn add_buildings() -> Self {
        Self {
            name: String::from("Add Buildings"),
            icon_category: String::from("action"),
            icon_name: String::from("home"),
        }
    }

    #[must_use]
    pub fn add_way_surface() -> Self {
        Self {
            name: String::from("Add Way Surface"),
            icon_category: String::from("maps"),
            icon_name: String::from("add_road"),
        }
    }

    #[must_use]
    pub fn get_icon_path(&self) -> String {
        format!(
            "icons/{}/{}/materialiconsoutlined/24dp/1x/outline_{}_black_24dp.png",
            self.icon_category, self.icon_name, self.icon_name
        )
    }

    #[must_use]
    pub fn get_fab(&self) -> String {
        format!(
            "icons/{}/{}/materialiconsoutlined/24dp/1x/outline_{}_black_24dp.png",
            self.icon_category, self.icon_name, self.icon_name
        )
    }

    #[allow(clippy::return_self_not_must_use)]
    pub fn spawn_fab(
        self,
        commands: &mut Commands,
        assets: &Res<AssetServer>,
        size: FloatingActionButtonSize,
        bar: Entity,
    ) -> Self {
        let icon = assets.load(self.get_icon_path());
        let button = FloatingActionButton::new(size, icon);
        button.spawn(commands, bar);
        self
    }
}
