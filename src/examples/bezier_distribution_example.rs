use crate::distribution::*;
use crate::infrastructure::Way;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

const ACCURACY: f32 = 1e-3;

pub struct BezierDistributionExample;

impl Plugin for BezierDistributionExample {
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
        let sizes = vec![
            Distributable {
                order: 0,
                size: Vec3::new(3.0, 2.0, 1.0),
                ..default()
            },
            Distributable {
                order: 1,
                size: Vec3::new(1.0, 3.5, 2.5),
                ..default()
            },
            Distributable {
                order: 2,
                size: Vec3::new(3.5, 2.5, 3.0),
                ..default()
            },
            Distributable {
                order: 3,
                size: Vec3::new(4.0, 2.0, 2.0),
                ..default()
            },
            Distributable {
                order: 4,
                size: Vec3::new(2.0, 3.0, 1.0),
                ..default()
            },
            Distributable {
                order: 5,
                size: Vec3::new(3.0, 5.0, 3.0),
                ..default()
            },
            Distributable {
                order: 6,
                size: Vec3::new(2.0, 2.0, 2.0),
                ..default()
            },
            Distributable {
                order: 7,
                size: Vec3::new(7.0, 3.0, 1.0),
                ..default()
            },
        ];
        let spline_length = way.spline.get_length(ACCURACY);
        let builder = FlexBuilder::new()
            .with_axis(Vec3::X, Vec3::Y)
            .with_bounds(Vec3::new(spline_length, 0.0, 0.0))
            .with_flex_wrap(FlexWrap::Wrap)
            .with_justify_content(JustifyContent::SpaceAround)
            .with_align_content(AlignContent::SpaceEvenly)
            .with_align_items_cross(AlignItems::Start)
            .with_align_items_normal(AlignItems::Start)
            .with_gap(Vec3::new(3.0, 10.0, 10.0));
        let container = builder.execute(sizes);
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
            let distance = item.translation.x + spline_length * 0.5;
            let param = way
                .spline
                .get_param_at_length(distance, ACCURACY)
                .expect("distance should be in range");
            let point = way.spline.get_point_at_param(param);
            let tangent = way.spline.get_tangent_at_param(param);
            let rotation = Quat::from_rotation_arc(Vec3::X, tangent);
            let transform = Transform::from_rotation(rotation);
            let translation = point + transform.transform_point(item.translation.with_x(0.0));
            let size = item.source.size;
            let bundle = (
                Mesh3d(meshes.add(Cuboid::from_size(size))),
                Transform::from_translation(translation).with_rotation(rotation),
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
