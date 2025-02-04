use crate::distribution::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub struct BezierDistributionExample;

impl Plugin for BezierDistributionExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, startup_system);
    }
}

fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sizes = vec![
        SourceItem {
            size: Vec3::new(3.0, 2.0, 1.0),
            ..default()
        },
        SourceItem {
            size: Vec3::new(1.0, 3.5, 2.5),
            ..default()
        },
        SourceItem {
            size: Vec3::new(3.5, 2.5, 3.0),
            ..default()
        },
        SourceItem {
            size: Vec3::new(4.0, 2.0, 2.0),
            ..default()
        },
        SourceItem {
            size: Vec3::new(2.0, 3.0, 1.0),
            ..default()
        },
        SourceItem {
            size: Vec3::new(3.0, 5.0, 3.0),
            ..default()
        },
        SourceItem {
            size: Vec3::new(2.0, 2.0, 2.0),
            ..default()
        },
        SourceItem {
            size: Vec3::new(7.0, 3.0, 1.0),
            ..default()
        },
    ];
    let builder = FlexBuilder::new()
        .with_axis(Vec3::X, Vec3::Y)
        .with_flex_wrap(FlexWrap::Wrap)
        .with_justify_content(JustifyContent::SpaceAround)
        .with_align_content(AlignContent::SpaceEvenly)
        .with_align_items_cross(AlignItems::Center)
        .with_align_items_normal(AlignItems::Start)
        .with_gap(Vec3::new(3.0, 10.0, 10.0));
    let container = builder.with_items(sizes).execute();
    let bundle = (
        Mesh3d(meshes.add(Cuboid::from_size(container.size))),
        Transform::from_translation(Vec3::new(0.0, 0.0, container.size.z * 0.5)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: tailwind::SKY_300.with_alpha(0.05).into(),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })),
    );
    let parent = commands.spawn(bundle).id();
    for item in container.items {
        let size = item.source.size;
        let bundle = (
            Mesh3d(meshes.add(Cuboid::from_size(size))),
            Transform::from_translation(item.translation),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: tailwind::RED_600.with_alpha(0.5).into(),
                perceptual_roughness: 1.0,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
        );
        commands.spawn(bundle).set_parent(parent);
    }
}
