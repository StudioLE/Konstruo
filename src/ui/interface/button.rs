use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use FloatingActionButtonSize::{Medium, Small};

const ENABLED: Srgba = tailwind::SLATE_400;
const HOVERED: Srgba = tailwind::BLUE_400;

/// A floating action button.
/// - <https://m3.material.io/components/floating-action-button/overview>
#[derive(Component)]
pub struct FloatingActionButton;

pub enum FloatingActionButtonSize {
    Small,
    Medium,
}

impl FloatingActionButton {
    pub fn spawn(
        commands: &mut Commands,
        size: FloatingActionButtonSize,
        icon: Handle<Image>,
        parent: Entity,
        mut action: Observer,
    ) {
        let radius = match size {
            Small => 12.0,
            Medium => 16.0,
        };
        let padding = match size {
            Small => 8.0,
            Medium => 16.0,
        };
        let margin = match size {
            Small => 8.0,
            Medium => 16.0,
        };
        let node = Node {
            padding: UiRect::all(Val::Px(padding)),
            margin: UiRect::all(Val::Px(margin)),
            min_height: Val::Px(24.0 + padding * 2.0),
            min_width: Val::Px(24.0 + padding * 2.0),
            overflow: Overflow::visible(),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        };
        let bundle = (
            node,
            BackgroundColor(ENABLED.into()),
            BorderRadius::all(Val::Px(radius)),
            FloatingActionButton,
        );
        let button = commands
            .spawn(bundle)
            .set_parent(parent)
            .observe(on_pointer_over)
            .observe(on_pointer_out)
            .id();
        action.watch_entity(button);
        commands.spawn(action);
        commands.spawn(ImageNode::new(icon)).set_parent(button);
    }
}

fn on_pointer_over(
    event: Trigger<Pointer<Over>>,
    mut query: Query<&mut BackgroundColor, With<FloatingActionButton>>,
) {
    let Ok(mut bg) = query.get_mut(event.target()) else {
        error!("Failed to get FloatingActionButton");
        return;
    };
    *bg = BackgroundColor(HOVERED.into());
}

fn on_pointer_out(
    event: Trigger<Pointer<Out>>,
    mut query: Query<&mut BackgroundColor, With<FloatingActionButton>>,
) {
    let Ok(mut bg) = query.get_mut(event.target()) else {
        error!("Failed to get FloatingActionButton");
        return;
    };
    *bg = BackgroundColor(ENABLED.into());
}
