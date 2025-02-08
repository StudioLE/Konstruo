use crate::distribution::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub struct FlexNestedExample;

impl Plugin for FlexNestedExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, startup_system);
    }
}

fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create and spawn the root container entity
    let flex = FlexBuilder::new()
        .with_axis(Vec3::X, Vec3::Y)
        .with_gap(Vec3::splat(2.0))
        .build();
    let bundle = (
        Distribution {
            flex,
            generate_container_mesh: true,
            translate_to_ground: true,
        },
        Mesh3d::default(),
        Transform::default(),
        MeshMaterial3d(get_container_material(&mut materials)),
    );
    let root = commands.spawn(bundle).id();

    // Create and spawn the first nested item
    let flex = FlexBuilder::new()
        .with_axis(Vec3::Y, Vec3::X)
        .with_align_items_cross(AlignItems::FlexEnd)
        .with_gap(Vec3::splat(0.5))
        .build();
    let bundle = (
        Distributable {
            order: 0,
            ..default()
        },
        Distribution { flex, ..default() },
        Mesh3d::default(),
        Transform::default(),
        MeshMaterial3d(get_container_material(&mut materials)),
    );
    let nested_item_0 = commands.spawn(bundle).set_parent(root).id();

    // Create and spawn the children of the the first nested item
    let material = get_items_0_material(&mut materials);
    for distributable in get_items_0() {
        let size = distributable.size.expect("size should be set");
        let bundle = (
            distributable,
            Mesh3d(meshes.add(Cuboid::from_size(size))),
            MeshMaterial3d(material.clone()),
        );
        commands.spawn(bundle).set_parent(nested_item_0);
    }

    // Create and spawn the second nested item
    let flex = FlexBuilder::new()
        .with_axis(Vec3::Z, Vec3::X)
        .with_gap(Vec3::splat(0.5))
        .build();
    let bundle = (
        Distributable {
            order: 1,
            ..default()
        },
        Distribution { flex, ..default() },
        Mesh3d::default(),
        Transform::default(),
        MeshMaterial3d(get_container_material(&mut materials)),
    );
    let nested_item_1 = commands.spawn(bundle).set_parent(root).id();

    // Create and spawn the children of the second nested item
    let material = get_items_1_material(&mut materials);
    for distributable in get_items_1() {
        let size = distributable.size.expect("size should be set");
        let bundle = (
            distributable,
            Mesh3d(meshes.add(Cuboid::from_size(size))),
            MeshMaterial3d(material.clone()),
        );
        commands.spawn(bundle).set_parent(nested_item_1);
    }
}

fn get_items_0() -> Vec<Distributable> {
    vec![
        Distributable {
            order: 0,
            size: Some(Vec3::new(3.0, 2.0, 1.0)),
            // margin: Vec3::splat(0.5),
            ..default()
        },
        Distributable {
            order: 1,
            size: Some(Vec3::new(1.0, 3.5, 2.5)),
            // margin: Vec3::splat(0.5),
            ..default()
        },
        Distributable {
            order: 2,
            size: Some(Vec3::new(3.5, 2.5, 3.0)),
            // margin: Vec3::splat(0.5),
            ..default()
        },
    ]
}

fn get_items_1() -> Vec<Distributable> {
    vec![
        Distributable {
            order: 0,
            size: Some(Vec3::new(4.0, 2.0, 2.0)),
            // margin: Vec3::splat(0.5),
            ..default()
        },
        Distributable {
            order: 1,
            size: Some(Vec3::new(2.0, 3.0, 1.0)),
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

fn get_items_0_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: tailwind::RED_600.with_alpha(0.5).into(),
        perceptual_roughness: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    })
}

fn get_items_1_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: tailwind::YELLOW_600.with_alpha(0.5).into(),
        perceptual_roughness: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    })
}
