use crate::distribution::{Distributable, FlexBuilder};
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub struct DistributionExample;

impl Plugin for DistributionExample {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, Self::startup_system);
    }
}

impl DistributionExample {
    fn startup_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let items = vec![
            Item {
                size: Vec3::new(3.0, 2.0, 1.0),
            },
            Item {
                size: Vec3::new(1.0, 3.5, 1.0),
            },
            Item {
                size: Vec3::new(3.5, 2.5, 1.0),
            },
            Item {
                size: Vec3::new(4.0, 2.0, 1.0),
            },
            Item {
                size: Vec3::new(2.0, 3.0, 1.0),
            },
        ];
        let layout = FlexBuilder::new()
            .with_axis(Vec3::X, Vec3::Y)
            .with_container(Vec3::new(10.0, 20.0, 30.0))
            .with_flex_wrap(FlexWrap::Wrap)
            .with_justify_content(JustifyContent::SpaceAround)
            .with_align_content(AlignContent::SpaceEvenly)
            .with_align_items(AlignItems::Center)
            .with_items(items)
            .execute();
        // assert_eq!(layout.items.len(), 3);
        info!("Parent size: {}", layout.size);
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
            let size = item.item.get_size();
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
}

#[derive(Component)]
#[require(Transform)]
pub struct Item {
    size: Vec3,
}

impl Distributable for Item {
    fn get_size(&self) -> Vec3 {
        self.size
    }
}
