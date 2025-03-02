use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use std::ops::DerefMut;
use FloatingActionButtonSize::*;

const ENABLED: Srgba = tailwind::SLATE_400;
const HOVERED: Srgba = tailwind::BLUE_400;
const PRESSED: Srgba = tailwind::RED_400;

/// A floating action button.
/// - <https://m3.material.io/components/floating-action-button/overview>
#[derive(Component)]
pub struct FloatingActionButton {
    pub size: FloatingActionButtonSize,
    pub icon: Handle<Image>,
    pub active: bool,
}

pub enum FloatingActionButtonSize {
    Small,
    Medium,
}

impl FloatingActionButton {
    #[must_use]
    pub fn new(size: FloatingActionButtonSize, icon: Handle<Image>) -> Self {
        Self {
            size,
            icon,
            active: false,
        }
    }

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
            BackgroundColor(ENABLED.into()),
            BorderRadius::all(Val::Px(radius)),
            self,
        );
        let button = commands
            .spawn(bundle)
            .set_parent(parent)
            .observe(on_pointer_over)
            .observe(on_pointer_out)
            .observe(on_pointer_click)
            .id();
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

fn on_pointer_over(
    event: Trigger<Pointer<Over>>,
    mut query: Query<(&mut BackgroundColor, &FloatingActionButton)>,
) {
    let Ok((mut bg, button)) = query.get_mut(event.entity()) else {
        error!("Failed to get FloatingActionButton");
        return;
    };
    if !button.active {
        *bg = BackgroundColor(HOVERED.into());
    }
}

fn on_pointer_out(
    event: Trigger<Pointer<Out>>,
    mut query: Query<(&mut BackgroundColor, &FloatingActionButton)>,
) {
    let Ok((mut bg, button)) = query.get_mut(event.entity()) else {
        error!("Failed to get FloatingActionButton");
        return;
    };
    if !button.active {
        *bg = BackgroundColor(ENABLED.into());
    }
}

fn on_pointer_click(
    event: Trigger<Pointer<Click>>,
    mut query: Query<(&mut BackgroundColor, &mut FloatingActionButton)>,
) {
    let Ok((mut bg, mut button)) = query.get_mut(event.entity()) else {
        error!("Failed to get FloatingActionButton");
        return;
    };
    if !button.active {
        *bg = BackgroundColor(PRESSED.into());
        button.deref_mut().active = true;
    }
}
