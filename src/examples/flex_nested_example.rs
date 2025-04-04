use crate::distribution::*;
use crate::examples::ExampleFactory;
use crate::geometry::Cuboid;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub struct FlexNestedExample;

impl Plugin for FlexNestedExample {
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

    // Create and spawn the root container entity
    let flex = FlexBuilder::new()
        .with_axis(Vec3::X, Vec3::Y)
        .with_gap(Vec3::splat(2.0))
        .build();
    let root = factory.spawn_container(flex);

    // Create and spawn the first item
    let item_0 = factory.item_0_bundle(root);
    let item_0 = factory.commands.spawn(item_0).id();

    // Create and spawn the children of the the first item
    let material = factory.get_material_0();
    for distributable in get_item_0_children() {
        let child = factory.nested_bundle(distributable, material.clone(), item_0);
        factory.commands.spawn(child);
    }

    // Create and spawn the second nested item;
    let item_1 = factory.item_1_bundle(root);
    let item_1 = factory.commands.spawn(item_1).id();

    // Create and spawn the children of the second nested item
    let material = factory.get_material_1();
    for distributable in get_item_1_children() {
        let child = factory.nested_bundle(distributable, material.clone(), item_1);
        factory.commands.spawn(child);
    }
}

impl ExampleFactory<'_> {
    fn item_0_bundle(&mut self, parent: Entity) -> impl Bundle {
        let flex = FlexBuilder::new()
            .with_axis(Vec3::Y, Vec3::X)
            .with_align_items_cross(AlignItems::FlexEnd)
            .with_gap(Vec3::splat(0.5))
            .build();
        let material = self.get_container_material();
        (
            Distributable {
                order: 0,
                ..default()
            },
            Distribution { flex, ..default() },
            Mesh3d::default(),
            Transform::default(),
            MeshMaterial3d(material),
            ChildOf { parent },
        )
    }

    fn item_1_bundle(&mut self, parent: Entity) -> impl Bundle {
        let flex = FlexBuilder::new()
            .with_axis(Vec3::Z, Vec3::X)
            .with_gap(Vec3::splat(0.5))
            .build();
        let material = self.get_container_material();
        (
            Distributable {
                order: 1,
                ..default()
            },
            Distribution { flex, ..default() },
            Mesh3d::default(),
            Transform::default(),
            MeshMaterial3d(material),
            ChildOf { parent },
        )
    }

    fn nested_bundle(
        &mut self,
        distributable: Distributable,
        material: Handle<StandardMaterial>,
        parent: Entity,
    ) -> impl Bundle {
        let size = distributable.size.expect("size should be set");
        let cuboid = Cuboid::new(Transform::from_scale(size));
        (
            distributable,
            Mesh3d(self.meshes.add(cuboid.get_triangles().to_mesh())),
            MeshMaterial3d(material.clone()),
            ChildOf { parent },
        )
    }

    fn get_container_material(&mut self) -> Handle<StandardMaterial> {
        self.materials.add(StandardMaterial {
            base_color: tailwind::SKY_300.with_alpha(0.05).into(),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })
    }

    fn get_material_0(&mut self) -> Handle<StandardMaterial> {
        self.materials.add(StandardMaterial {
            base_color: tailwind::RED_600.with_alpha(0.5).into(),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })
    }

    fn get_material_1(&mut self) -> Handle<StandardMaterial> {
        self.materials.add(StandardMaterial {
            base_color: tailwind::YELLOW_600.with_alpha(0.5).into(),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })
    }
}

fn get_item_0_children() -> Vec<Distributable> {
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

fn get_item_1_children() -> Vec<Distributable> {
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
