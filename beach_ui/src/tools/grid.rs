use bevy::color::palettes::*;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

const LENGTH: i32 = 1000;
const SPACING: [i32; 3] = [1, 10, 100];

#[allow(clippy::integer_division)]
const COUNT: i32 = LENGTH / SPACING[0];

#[derive(Component)]
pub enum Grid {
    Minor,
    Standard,
    Major,
}

#[allow(clippy::cast_precision_loss)]
/// Spawn an [`AxisGizmo`] entity with child meshes for each axis.
pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let range = 0..COUNT;
    let max = LENGTH as f32 / 2.0;
    let min = max * -1.0;
    let mut vertices = [Vec::new(), Vec::new(), Vec::new()];
    for i in range {
        let x = i as f32;
        let start_x = vec3(min + x, min, 0.0);
        let end_x = vec3(min + x, max, 0.0);
        let start_z = vec3(min, min + x, 0.0);
        let end_z = vec3(max, min + x, 0.0);
        if i % SPACING[2] == 0 {
            vertices[2].push(start_x);
            vertices[2].push(end_x);
            vertices[2].push(start_z);
            vertices[2].push(end_z);
        } else if i % SPACING[1] == 0 {
            vertices[1].push(start_x);
            vertices[1].push(end_x);
            vertices[1].push(start_z);
            vertices[1].push(end_z);
        } else {
            vertices[0].push(start_x);
            vertices[0].push(end_x);
            vertices[0].push(start_z);
            vertices[0].push(end_z);
        }
    }
    let material0 = StandardMaterial {
        emissive: basic::WHITE.into(),
        base_color: basic::WHITE.with_alpha(0.05).into(),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    };
    let material1 = StandardMaterial {
        emissive: basic::WHITE.into(),
        base_color: basic::WHITE.with_alpha(0.2).into(),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    };
    let material2 = StandardMaterial {
        emissive: basic::WHITE.into(),
        base_color: basic::WHITE.with_alpha(0.35).into(),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    };
    let mesh0 = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices[0].clone());
    let mesh1 = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices[1].clone());
    let mesh2 = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices[2].clone());
    let bundle0 = (
        Grid::Minor,
        Mesh3d(meshes.add(mesh0)),
        MeshMaterial3d(materials.add(material0)),
    );
    let bundle1 = (
        Grid::Standard,
        Mesh3d(meshes.add(mesh1)),
        MeshMaterial3d(materials.add(material1)),
    );
    let bundle2 = (
        Grid::Major,
        Mesh3d(meshes.add(mesh2)),
        MeshMaterial3d(materials.add(material2)),
    );
    commands.spawn(bundle0);
    commands.spawn(bundle1);
    commands.spawn(bundle2);
}
