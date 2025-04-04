use crate::distribution::*;
use crate::examples::ExampleFactory;
use bevy::prelude::*;

pub struct FlexWrappingExample;

impl Plugin for FlexWrappingExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, startup_system);
    }
}

fn startup_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut factory = ExampleFactory {
        commands,
        meshes,
        materials,
    };
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
    let distribution_entity = factory.spawn_container(flex);
    let items = get_items();
    factory.spawn_items(items, distribution_entity);
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
