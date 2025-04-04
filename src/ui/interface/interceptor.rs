use crate::ui::*;
use bevy::color::palettes::basic::BLACK;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

const FRAME_COLOR: Srgba = tailwind::SLATE_400;

/// Intercept pointer events.
#[derive(Component)]
pub struct Interceptor;

/// A UI frame that intercepts pointer events and displays a label indicating the active mode.
#[derive(Component)]
pub struct InterceptorLabel;

impl Interceptor {
    /// System to spawn [`Interceptor`] on startup.
    pub(super) fn startup_system(
        mut commands: Commands,
        query: Query<Entity, With<PrimaryCamera>>,
        assets: Res<AssetServer>,
    ) {
        let Ok(camera) = query.single() else {
            warn!("Failed to get PrimaryCamera");
            return;
        };
        let font = assets.load(DEFAULT_FONT);
        commands
            .spawn(interceptor_bundle(camera))
            .with_children(|commands| {
                commands.spawn(frame_bundle()).with_children(|commands| {
                    commands
                        .spawn(label_container_bundle())
                        .with_child(label_text_bundle(font));
                });
            })
            .observe(Drawing::on_pointer_down)
            .observe(Drawing::on_pointer_up);
    }

    /// System to update the [`Interceptor`] visibility when [`InterfaceState`] is changed.
    pub(super) fn update_system(
        interface: Res<InterfaceState>,
        mut interceptors: Query<&mut Visibility, With<Interceptor>>,
        mut labels: Query<&mut Text, With<InterceptorLabel>>,
    ) {
        if !interface.is_changed() {
            return;
        }
        let Ok(mut visibility) = interceptors.single_mut() else {
            warn!("Failed to get Visibility of Interceptor");
            return;
        };
        let Ok(mut text) = labels.single_mut() else {
            warn!("Failed to get Visibility of Interceptor");
            return;
        };
        *visibility = match *interface {
            InterfaceState::DrawPath => Visibility::Visible,
            _ => Visibility::Hidden,
        };
        *text = match *interface {
            InterfaceState::DrawPath => Text::new("Drawing"),
            _ => Text::new("Default"),
        };
    }
}

#[must_use]
fn interceptor_bundle(camera: Entity) -> impl Bundle {
    (
        Interceptor,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        Pickable::default(),
        ZIndex(INTERCEPTOR_Z),
        UiTargetCamera(camera),
        Visibility::Hidden,
    )
}

#[must_use]
fn frame_bundle() -> impl Bundle {
    (
        Node {
            border: UiRect::all(Val::Px(8.0)),
            flex_grow: 1.0,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            ..default()
        },
        BorderColor::from(FRAME_COLOR),
        BorderRadius::all(Val::Px(32.0)),
    )
}

#[must_use]
fn label_container_bundle() -> impl Bundle {
    (
        Node {
            padding: UiRect::px(8.0, 8.0, 4.0, 4.0),
            ..default()
        },
        BackgroundColor::from(FRAME_COLOR),
        BorderColor::from(FRAME_COLOR),
        BorderRadius::px(8.0, 8.0, 0.0, 0.0),
    )
}

#[must_use]
fn label_text_bundle(font: Handle<Font>) -> impl Bundle {
    (
        InterceptorLabel,
        Text::new("Drawing"),
        TextColor::from(BLACK),
        TextFont {
            font,
            font_size: 16.0,
            ..default()
        },
    )
}
