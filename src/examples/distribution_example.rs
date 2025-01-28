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
    let sizes = vec![
        Vec3::new(3.0, 2.0, 1.0),
        Vec3::new(1.0, 3.5, 2.5),
        Vec3::new(3.5, 2.5, 3.0),
        Vec3::new(4.0, 2.0, 2.0),
        Vec3::new(2.0, 3.0, 1.0),
    ];
    let builder = match PATTERN {
        Wrapped => FlexBuilder::new()
            .with_axis(Vec3::X, Vec3::Y)
            .with_container(Vec3::new(10.0, 20.0, 30.0))
            .with_flex_wrap(FlexWrap::Wrap)
            .with_justify_content(JustifyContent::SpaceAround)
            .with_align_content(AlignContent::SpaceEvenly)
            .with_align_items(AlignItems::Center),
        Stacked => FlexBuilder::new().with_axis(Vec3::Z, Vec3::X),
    };
    let layout = builder.with_items(sizes).execute();
    let bundle = (
        Mesh3d(meshes.add(Cuboid::from_size(layout.size.with_z(0.1)))),
        Transform::from_translation(Vec3::ZERO),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: tailwind::BLUE_400.with_alpha(0.1).into(),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
    );
    let parent = commands.spawn(bundle).id();
    for item in layout.items {
        let size = item.original_size;
        let bundle = (
            Mesh3d(meshes.add(Cuboid::from_size(size))),
            Transform::from_translation(item.translation),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: tailwind::RED_400.with_alpha(0.5).into(),
                perceptual_roughness: 1.0,
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
        );
        commands.spawn(bundle).set_parent(parent);
    }
}
