use crate::ui::{FloatingActionButton, FloatingActionButtonSize};
use bevy::prelude::*;

pub struct Action {
    pub name: String,
    pub icon: Icon,
}

pub enum Icon {
    Material { category: String, name: String },
    FontAwesome { name: String },
}

impl Action {
    #[must_use]
    pub fn deselect() -> Self {
        Self {
            name: String::from("Deselect"),
            icon: Icon::FontAwesome {
                name: String::from("times"),
            },
        }
    }

    #[must_use]
    pub fn draw_way() -> Self {
        Self {
            name: String::from("Draw Way"),
            icon: Icon::FontAwesome {
                name: String::from("bezier-curve"),
            },
        }
    }

    #[must_use]
    pub fn more() -> Self {
        Self {
            name: String::from("More"),
            icon: Icon::FontAwesome {
                name: String::from("ellipsis-v-alt"),
            },
        }
    }

    #[must_use]
    pub fn settings() -> Self {
        Self {
            name: String::from("Settings"),
            icon: Icon::FontAwesome {
                name: String::from("cog"),
            },
        }
    }

    #[must_use]
    pub fn edit() -> Self {
        Self {
            name: String::from("Edit"),
            icon: Icon::FontAwesome {
                name: String::from("edit"),
            },
        }
    }

    #[must_use]
    pub fn remove() -> Self {
        Self {
            name: String::from("Remove"),
            icon: Icon::FontAwesome {
                name: String::from("trash"),
            },
        }
    }

    #[must_use]
    pub fn info() -> Self {
        Self {
            name: String::from("Info"),
            icon: Icon::FontAwesome {
                name: String::from("info"),
            },
        }
    }

    #[must_use]
    pub fn add_buildings() -> Self {
        Self {
            name: String::from("Add Buildings"),
            icon: Icon::FontAwesome {
                name: String::from("home"),
            },
        }
    }

    #[must_use]
    pub fn add_way_surface() -> Self {
        Self {
            name: String::from("Add Way Surface"),
            icon: Icon::FontAwesome {
                name: String::from("road"),
            },
        }
    }

    #[must_use]
    pub fn get_icon_path(&self) -> String {
        match &self.icon {
            Icon::Material { category, name } => format!(
                "material-icons/{category}/{name}/materialiconsoutlined/24dp/1x/outline_{name}_black_24dp.png"
            ),
            Icon::FontAwesome { name } => format!(
                "font-awesome/24px/{name}.png",
            ),
        }
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
