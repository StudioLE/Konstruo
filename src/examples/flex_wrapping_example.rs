use crate::distribution::*;
use crate::examples::ExampleHelpers;
use bevy::prelude::*;

pub struct FlexWrappingExample;

impl Plugin for FlexWrappingExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, startup_system);
    }
}

fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let flex = FlexBuilder::new()
        .with_axis(Vec3::X, Vec3::Y)
        .with_bounds(Vec3::new(10.0, 20.0, 30.0))
        .with_flex_wrap(FlexWrap::Wrap)
        .with_justify_content(JustifyContent::SpaceAround)
        .with_align_content(AlignContent::SpaceEvenly)
        .with_align_items_cross(AlignItems::Center)
        .with_align_items_normal(AlignItems::Start)
        .with_gap(Vec3::new(0.5, 0.5, 3.0))
        .build();
    let distribution_entity =
        ExampleHelpers::spawn_container(&mut commands, &mut meshes, &mut materials, flex);
    let items = get_items();
    ExampleHelpers::spawn_items(
        &mut commands,
        &mut meshes,
        &mut materials,
        items,
        distribution_entity,
    );
}

fn get_items() -> Vec<Distributable> {
    vec![
        Distributable {
            order: 0,
            size: Some(Vec3::new(3.0, 2.0, 1.0)),
            ..default()
        },
        Distributable {
            order: 1,
            size: Some(Vec3::new(1.0, 3.5, 2.5)),
            ..default()
        },
        Distributable {
            order: 2,
            size: Some(Vec3::new(3.5, 2.5, 3.0)),
            ..default()
        },
        Distributable {
            order: 3,
            size: Some(Vec3::new(4.0, 2.0, 2.0)),
            ..default()
        },
        Distributable {
            order: 4,
            size: Some(Vec3::new(2.0, 3.0, 1.0)),
            ..default()
        },
    ]
}
