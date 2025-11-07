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
    #[allow(clippy::must_use_candidate)]
    pub fn spawn(
        commands: &mut Commands,
        size: FloatingActionButtonSize,
        icon: Handle<Image>,
        parent: Entity,
        mut action: Observer,
    ) -> Entity {
        let button = commands
            .spawn(Self::button_bundle(size, parent))
            .observe(on_pointer_over)
            .observe(on_pointer_out)
            .with_child(Self::icon_bundle(icon))
            .id();
        action.watch_entity(button);
        commands.spawn(action);
        button
    }

    fn button_bundle(size: FloatingActionButtonSize, parent: Entity) -> impl Bundle {
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
        (
            FloatingActionButton,
            ChildOf(parent),
            Node {
                padding: UiRect::all(Val::Px(padding)),
                margin: UiRect::all(Val::Px(margin)),
                min_height: Val::Px(24.0 + padding * 2.0),
                min_width: Val::Px(24.0 + padding * 2.0),
                overflow: Overflow::visible(),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(ENABLED.into()),
            BorderRadius::all(Val::Px(radius)),
        )
    }

    fn icon_bundle(icon: Handle<Image>) -> impl Bundle {
        ImageNode::new(icon)
    }
}

fn on_pointer_over(
    trigger: Trigger<Pointer<Over>>,
    mut query: Query<&mut BackgroundColor, With<FloatingActionButton>>,
) {
    let Ok(mut bg) = query.get_mut(trigger.target()) else {
        error!("Failed to get FloatingActionButton");
        return;
    };
    *bg = BackgroundColor(HOVERED.into());
}

fn on_pointer_out(
    trigger: Trigger<Pointer<Out>>,
    mut query: Query<&mut BackgroundColor, With<FloatingActionButton>>,
) {
    let Ok(mut bg) = query.get_mut(trigger.target()) else {
        error!("Failed to get FloatingActionButton");
        return;
    };
    *bg = BackgroundColor(ENABLED.into());
}
