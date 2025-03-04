pub enum Icon {
    Material { category: String, name: String },
    FontAwesome { name: String },
}

impl Icon {
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
