#[derive(Clone, Debug)]
pub enum Icon {
    Material { category: String, name: String },
    FontAwesome { name: String },
}

impl Icon {
    /// Create a new [`Icon::FontAwesome`].
    #[must_use]
    pub fn font_awesome(name: &str) -> Icon {
        Icon::FontAwesome {
            name: name.to_owned(),
        }
    }

    /// Create a new [`Icon::Material`].
    #[must_use]
    pub fn material(category: &str, name: &str) -> Icon {
        Icon::Material {
            category: category.to_owned(),
            name: name.to_owned(),
        }
    }

    /// Get the path to the icon asset.
    #[must_use]
    pub fn get_path(&self) -> String {
        match &self {
            Icon::Material { category, name } => format!(
                "material-icons/{category}/{name}/materialiconsoutlined/24dp/1x/outline_{name}_black_24dp.png"
            ),
            Icon::FontAwesome { name } => format!(
                "font-awesome/24px/{name}.png",
            ),
        }
    }
}
