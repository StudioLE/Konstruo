use crate::examples::ExampleMaterials;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use konstruo_distribution::*;
use konstruo_paths::Path;

const ACCURACY: f32 = 1e-3;

pub struct FlexAlongBezierExample;

/// Factory to spawn the example.
struct Factory<'w> {
    pub commands: Commands<'w, 'w>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
}

impl Plugin for FlexAlongBezierExample {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, path_added_system);
    }
}

impl Factory<'_> {
    fn spawn(&mut self, entity: Entity, path: &Path) -> Entity {
        let distribution = self.distribution_bundle(path, entity);
        let distributables: Vec<_> = get_items()
            .into_iter()
            .map(|distributable| self.distributable_bundle(distributable))
            .collect();
        self.commands
            .spawn(distribution)
            .with_children(|commands| {
                for bundle in distributables {
                    commands.spawn(bundle);
                }
            })
            .id()
    }

    fn distribution_bundle(&mut self, path: &Path, parent: Entity) -> impl Bundle {
        let spline_length = path.spline.get_length(ACCURACY);
        let flex = FlexBuilder::new()
            .with_axis(Vec3::X, Vec3::Y)
            .with_bounds(Vec3::new(spline_length, 0.0, 0.0))
            .with_flex_wrap(FlexWrap::Wrap)
            .with_justify_content(JustifyContent::SpaceAround)
            .with_align_content(AlignContent::SpaceEvenly)
            .with_align_items_cross(AlignItems::Start)
            .with_align_items_normal(AlignItems::Start)
            .with_gap(Vec3::new(3.0, 10.0, 10.0))
            .build();
        let material = StandardMaterial {
            base_color: tailwind::SKY_300.with_alpha(0.05).into(),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        };
        (
            Distribution {
                flex,
                spline: Some(path.spline.clone()),
                translate_to_ground: true,
                ..default()
            },
            MeshMaterial3d(self.materials.add(material)),
            ChildOf { parent },
        )
    }

    fn distributable_bundle(&mut self, distributable: Distributable) -> impl Bundle {
        let size = distributable.size.expect("size should be set");
        (
            distributable,
            Mesh3d(self.meshes.add(Cuboid::from_size(size))),
            MeshMaterial3d(self.materials.add(ExampleMaterials::red_face())),
        )
    }
}

fn path_added_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Path), Added<Path>>,
) {
    let mut factory = Factory {
        commands,
        meshes,
        materials,
    };
    for (entity, path) in query.iter() {
        factory.spawn(entity, path);
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
