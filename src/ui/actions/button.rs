use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use FloatingActionButtonSize::*;

/// A floating action button.
/// - <https://m3.material.io/components/floating-action-button/overview>
#[derive(Component)]
pub struct FloatingActionButton {
    pub size: FloatingActionButtonSize,
    pub icon: Handle<Image>,
}

pub enum FloatingActionButtonSize {
    Small,
    Medium,
}

impl FloatingActionButton {
    pub fn spawn(self, commands: &mut Commands, parent: Entity) {
        let radius = match self.size {
            Small => 12.0,
            Medium => 16.0,
        };
        let node = match self.size {
            Small => Node {
                padding: UiRect::all(Val::Px(8.0)),
                overflow: Overflow::visible(),
                margin: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            Medium => Node {
                padding: UiRect::all(Val::Px(16.0)),
                overflow: Overflow::visible(),
                margin: UiRect::all(Val::Px(16.0)),
                ..default()
            },
        };
        let icon = self.icon.clone();
        let bundle = (
            node,
            BackgroundColor(tailwind::SLATE_400.into()),
            BorderRadius::all(Val::Px(radius)),
            self,
        );
        let button = commands.spawn(bundle).set_parent(parent).id();
        let icon = (
            ImageNode::new(icon),
            Node {
                height: Val::Px(24.0),
                width: Val::Px(24.0),
                ..default()
            },
        );
        commands.spawn(icon).set_parent(button);
    }
}
