use crate::examples::ExampleFactory;
use bevy::prelude::*;
use konstruo_distribution::*;

pub struct FlexVerticallyExample;

impl Plugin for FlexVerticallyExample {
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
        .with_axis(Vec3::Z, Vec3::X)
        .with_align_items_cross(AlignItems::Center)
        .with_align_items_normal(AlignItems::Center)
        .with_gap(Vec3::new(1.5, 1.0, 0.5))
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
