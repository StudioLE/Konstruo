use bevy::color::palettes::tailwind;
use bevy::prelude::*;

/// Plugin to enable Floating Action Buttons (FAB).
pub struct FloatingActionPlugin;

impl Plugin for FloatingActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl FloatingActionPlugin {
    fn startup_system(mut commands: Commands) {
        let bundle = (
            Camera2d,
            Camera {
                order: 4,
                ..default()
            },
        );
        commands.spawn(bundle);
        let bundle = (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // width: Val::Px(1000.0),
                // height: Val::Px(1000.0),
                // align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_content: AlignContent::End,
                align_items: AlignItems::End,
                flex_wrap: FlexWrap::NoWrap,
                // column_gap: Val::Px(16.0),
                // row_gap: Val::Px(16.0),
                ..default()
            },
            // BackgroundColor(tailwind::BLUE_200.into()),
        );
        let container = commands.spawn(bundle).id();
        for _i in 0..4 {
            let bundle = (
                // - <https://m3.material.io/components/floating-action-button/specs#7712fa7f-cd29-4852-86a9-fa2f4a01f6bc>
                Node {
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    // border: UiRect::all(Val::Px(5.0)),
                    margin: UiRect::new(Val::Px(16.0), Val::Px(16.0), Val::Px(0.0), Val::Px(16.0)),
                    // margin: UiRect::all(Val::Px(16.0)),
                    // horizontally center child text
                    // justify_content: JustifyContent::Center,
                    // vertically center child text
                    // align_items: AlignItems::Center,
                    flex_shrink: 0.0,
                    ..default()
                },
                BackgroundColor(tailwind::SKY_600.into()),
                BorderRadius::all(Val::Px(8.0)),
            );
            commands.spawn(bundle).set_parent(container);
        }
    }
}
