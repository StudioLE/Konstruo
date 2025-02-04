use crate::distribution::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use Pattern::*;

const PATTERN: Pattern = Wrapped;

#[allow(dead_code)]
enum Pattern {
    Wrapped,
    Stacked,
}

pub struct DistributionExample;

impl Plugin for DistributionExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, startup_system);
    }
}

fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let bundle = (
        Distribution {
            flex: get_factory(),
            generate_container_mesh: true,
            translate_to_ground: true,
        },
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        Transform::default(),
        MeshMaterial3d(get_container_material(&mut materials)),
    );
    let parent = commands.spawn(bundle).id();
    let material = get_item_material(&mut materials);
    for distributable in get_items() {
        let size = distributable.size;
        let bundle = (
            distributable,
            Mesh3d(meshes.add(Cuboid::from_size(size))),
            MeshMaterial3d(material.clone()),
        );
        commands.spawn(bundle).set_parent(parent);
    }
}

fn get_factory() -> FlexFactory {
    match PATTERN {
        Wrapped => FlexBuilder::new()
            .with_axis(Vec3::X, Vec3::Y)
            .with_bounds(Vec3::new(10.0, 20.0, 30.0))
            .with_flex_wrap(FlexWrap::Wrap)
            .with_justify_content(JustifyContent::SpaceAround)
            .with_align_content(AlignContent::SpaceEvenly)
            .with_align_items_cross(AlignItems::Center)
            .with_align_items_normal(AlignItems::Start)
            .with_gap(Vec3::new(0.5, 0.5, 3.0))
            .build(),
        Stacked => FlexBuilder::new()
            .with_axis(Vec3::Z, Vec3::X)
            .with_align_items_cross(AlignItems::Center)
            .with_align_items_normal(AlignItems::Center)
            .with_gap(Vec3::new(1.5, 1.0, 0.5))
            .build(),
    }
}

fn get_items() -> Vec<Distributable> {
    vec![
        Distributable {
            order: 0,
            size: Vec3::new(3.0, 2.0, 1.0),
            // margin: Vec3::splat(0.5),
            ..default()
        },
        Distributable {
            order: 1,
            size: Vec3::new(1.0, 3.5, 2.5),
            // margin: Vec3::splat(0.5),
            ..default()
        },
        Distributable {
            order: 2,
            size: Vec3::new(3.5, 2.5, 3.0),
            // margin: Vec3::splat(0.5),
            ..default()
        },
        Distributable {
            order: 3,
            size: Vec3::new(4.0, 2.0, 2.0),
            // margin: Vec3::splat(0.5),
            ..default()
        },
        Distributable {
            order: 4,
            size: Vec3::new(2.0, 3.0, 1.0),
            // margin: Vec3::splat(0.5),
            ..default()
        },
    ]
}

fn get_container_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: tailwind::SKY_300.with_alpha(0.05).into(),
        perceptual_roughness: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    })
}

fn get_item_material(materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: tailwind::RED_600.with_alpha(0.5).into(),
        perceptual_roughness: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    })
}
