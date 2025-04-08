use crate::examples::ExampleMaterials;
use bevy::prelude::*;
use konstruo_distribution::*;
use konstruo_geometry::{Cuboid, Edge, Solid};

pub struct ExampleFactory<'w> {
    pub commands: Commands<'w, 'w>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
}

impl ExampleFactory<'_> {
    pub(super) fn spawn_container(&mut self, flex: FlexFactory) -> Entity {
        let solid_bundle = self.container_solid_bundle();
        let edge_bundle = self.container_edges_bundle();
        self.commands
            .spawn(Self::container_bundle(flex))
            .with_child(solid_bundle)
            .with_child(edge_bundle)
            .id()
    }

    pub(super) fn spawn_items(&mut self, items: Vec<Distributable>, distribution_entity: Entity) {
        let material = self.materials.add(ExampleMaterials::red_face());
        let edge_material = self.materials.add(ExampleMaterials::red_edge());
        for distributable in items {
            self.spawn_item(
                material.clone(),
                edge_material.clone(),
                distributable,
                distribution_entity,
            );
        }
    }

    #[allow(clippy::must_use_candidate)]
    pub(super) fn spawn_item(
        &mut self,
        material: Handle<StandardMaterial>,
        edge_material: Handle<StandardMaterial>,
        distributable: Distributable,
        distribution_entity: Entity,
    ) -> Entity {
        let size = distributable.size.expect("size should be set");
        let cuboid = Cuboid::new(Transform::from_scale(size));
        let item_bundle = self.item_bundle(distributable, &cuboid, material, distribution_entity);
        let edges_bundle = self.item_edges_bundle(&cuboid, edge_material);
        self.commands
            .spawn(item_bundle)
            .with_child(edges_bundle)
            .id()
    }

    fn container_bundle(flex: FlexFactory) -> impl Bundle {
        (Distribution {
            flex,
            translate_to_ground: true,
            ..default()
        },)
    }

    fn container_solid_bundle(&mut self) -> impl Bundle {
        (
            Solid,
            DiagnosticContainer,
            Mesh3d(self.meshes.add(Cuboid::default().get_triangles().to_mesh())),
            MeshMaterial3d(
                self.materials
                    .add(ExampleMaterials::blue_face_transparent()),
            ),
        )
    }

    fn container_edges_bundle(&mut self) -> impl Bundle {
        (
            Edge,
            DiagnosticContainer,
            Mesh3d(self.meshes.add(Cuboid::default().get_edges().to_mesh())),
            MeshMaterial3d(self.materials.add(ExampleMaterials::blue_edge())),
        )
    }

    fn item_bundle(
        &mut self,
        distributable: Distributable,
        cuboid: &Cuboid,
        material: Handle<StandardMaterial>,
        parent: Entity,
    ) -> impl Bundle {
        (
            distributable,
            Solid,
            Mesh3d(self.meshes.add(cuboid.clone().get_triangles().to_mesh())),
            MeshMaterial3d(material),
            ChildOf { parent },
        )
    }

    fn item_edges_bundle(
        &mut self,
        cuboid: &Cuboid,
        material: Handle<StandardMaterial>,
    ) -> impl Bundle {
        (
            Edge,
            Mesh3d(self.meshes.add(cuboid.get_edges().to_mesh())),
            MeshMaterial3d(material),
        )
    }
}
