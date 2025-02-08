use crate::distribution::*;
use crate::infrastructure::Way;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

const ACCURACY: f32 = 1e-3;

pub struct FlexAlongBezierExample;

impl Plugin for FlexAlongBezierExample {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, way_added_system);
    }
}

fn way_added_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Way), Added<Way>>,
) {
    for (_entity, way) in query.iter() {
        let spline_length = way.spline.get_length(ACCURACY);
        let bundle = (
            Distribution {
                flex: FlexBuilder::new()
                    .with_axis(Vec3::X, Vec3::Y)
                    .with_bounds(Vec3::new(spline_length, 0.0, 0.0))
                    .with_flex_wrap(FlexWrap::Wrap)
                    .with_justify_content(JustifyContent::SpaceAround)
                    .with_align_content(AlignContent::SpaceEvenly)
                    .with_align_items_cross(AlignItems::Start)
                    .with_align_items_normal(AlignItems::Start)
                    .with_gap(Vec3::new(3.0, 10.0, 10.0))
                    .build(),
                spline: Some(way.spline.clone()),
                translate_to_ground: true,
                ..default()
            },
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: tailwind::SKY_300.with_alpha(0.05).into(),
                perceptual_roughness: 1.0,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
        );
        let parent = commands.spawn(bundle).id();
        for distributable in get_items() {
            let size = distributable.size.expect("size should be set");
            let bundle = (
                distributable,
                Mesh3d(meshes.add(Cuboid::from_size(size))),
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
        Distributable {
            order: 5,
            size: Some(Vec3::new(3.0, 5.0, 3.0)),
            ..default()
        },
        Distributable {
            order: 6,
            size: Some(Vec3::new(2.0, 2.0, 2.0)),
            ..default()
        },
        Distributable {
            order: 7,
            size: Some(Vec3::new(7.0, 3.0, 1.0)),
            ..default()
        },
    ]
}
