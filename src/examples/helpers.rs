use crate::distribution::{DiagnosticContainer, Distributable, Distribution, FlexFactory};
use crate::examples::ExampleMaterials;
use crate::geometry::Cuboid;
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
            Mesh3d(meshes.add(Cuboid::default().get_triangles().to_mesh())),
            MeshMaterial3d(materials.add(ExampleMaterials::blue_face_transparent())),
        );
        commands.spawn(bundle).set_parent(distribution_entity);
        let bundle = (
            DiagnosticContainer,
            Transform::default(),
            Mesh3d(meshes.add(Cuboid::default().get_edges().to_mesh())),
            MeshMaterial3d(materials.add(ExampleMaterials::blue_edge())),
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
        let material = materials.add(ExampleMaterials::red_face());
        let edge_material = materials.add(ExampleMaterials::red_edge());
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
        let cuboid = Cuboid::new(Transform::from_scale(size));
        let bundle = (
            distributable,
            Mesh3d(meshes.add(cuboid.clone().get_triangles().to_mesh())),
            MeshMaterial3d(material),
        );
        let item = commands.spawn(bundle).set_parent(distribution_entity).id();
        let bundle = (
            Mesh3d(meshes.add(cuboid.get_edges().to_mesh())),
            MeshMaterial3d(edge_material),
        );
        commands.spawn(bundle).set_parent(item);
        item
    }
}
