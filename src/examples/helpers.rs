use crate::distribution::{DiagnosticContainer, Distributable, Distribution, FlexFactory};
use crate::geometry::Cuboid;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub struct ExampleHelpers;

impl ExampleHelpers {
    pub(super) fn spawn_container(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        flex: FlexFactory,
    ) -> Entity {
        let bundle = (Distribution {
            flex,
            translate_to_ground: true,
            ..default()
        },);
        let distribution_entity = commands.spawn(bundle).id();
        let bundle = (
            DiagnosticContainer,
            Transform::default(),
            Mesh3d(meshes.add(Cuboid::default().to_triangles().to_mesh())),
            MeshMaterial3d(Self::get_container_material(materials)),
        );
        commands.spawn(bundle).set_parent(distribution_entity);
        let bundle = (
            DiagnosticContainer,
            Transform::default(),
            Mesh3d(meshes.add(Cuboid::default().to_edges().to_mesh())),
            MeshMaterial3d(Self::get_container_edge_material(materials)),
        );
        commands.spawn(bundle).set_parent(distribution_entity);
        distribution_entity
    }

    pub(super) fn spawn_items(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        items: Vec<Distributable>,
        distribution_entity: Entity,
    ) {
        let material = Self::get_item_material(materials);
        let edge_material = Self::get_item_edge_material(materials);
        for distributable in items {
            let _entity = Self::spawn_item(
                commands,
                meshes,
                material.clone(),
                edge_material.clone(),
                distributable,
                distribution_entity,
            );
        }
    }

    pub(super) fn spawn_item(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<StandardMaterial>,
        edge_material: Handle<StandardMaterial>,
        distributable: Distributable,
        distribution_entity: Entity,
    ) -> Entity {
        let size = distributable.size.expect("size should be set");
        let cuboid = Cuboid::new(size);
        let bundle = (
            distributable,
            Mesh3d(meshes.add(cuboid.clone().to_triangles().to_mesh())),
            MeshMaterial3d(material),
        );
        let item = commands.spawn(bundle).set_parent(distribution_entity).id();
        let bundle = (
            Mesh3d(meshes.add(cuboid.to_edges().to_mesh())),
            MeshMaterial3d(edge_material),
        );
        commands.spawn(bundle).set_parent(item);
        item
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

    fn get_container_edge_material(
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color: tailwind::SKY_300.into(),
            perceptual_roughness: 1.0,
            unlit: true,
            ..default()
        })
    }

    fn get_item_material(
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

    fn get_item_edge_material(
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color: tailwind::RED_600.into(),
            perceptual_roughness: 1.0,
            unlit: true,
            ..default()
        })
    }
}
