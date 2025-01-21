use crate::grid::GridMaterials;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

const LENGTH: i32 = 1000;
const SPACING: [i32; 3] = [1, 10, 100];

#[allow(clippy::integer_division)]
const COUNT: i32 = LENGTH / SPACING[0] + 1;

#[derive(Component)]
pub(super) enum Grid {
    Minor,
    Medium,
    Major,
}

impl Grid {
    #[allow(clippy::cast_precision_loss)]
    /// System to spawn a [`Grid`].
    pub(super) fn startup_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<GridMaterials>,
    ) {
        let (minor, medium, major) = create_lines();
        let minor = (
            Grid::Minor,
            create_mesh(&mut meshes, minor),
            MeshMaterial3d(materials.minor.clone()),
        );
        let medium = (
            Grid::Medium,
            create_mesh(&mut meshes, medium),
            MeshMaterial3d(materials.medium.clone()),
        );
        let major = (
            Grid::Major,
            create_mesh(&mut meshes, major),
            MeshMaterial3d(materials.major.clone()),
        );
        commands.spawn(minor);
        commands.spawn(medium);
        commands.spawn(major);
    }
}

fn create_lines() -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec3>) {
    let range = 0..COUNT;
    let max = LENGTH as f32 / 2.0;
    let min = max * -1.0;
    let mut minor = Vec::new();
    let mut medium = Vec::new();
    let mut major = Vec::new();
    for i in range {
        let x = i as f32;
        let start_x = vec3(min + x, min, 0.0);
        let end_x = vec3(min + x, max, 0.0);
        let start_z = vec3(min, min + x, 0.0);
        let end_z = vec3(max, min + x, 0.0);
        if i % SPACING[2] == 0 {
            major.push(start_x);
            major.push(end_x);
            major.push(start_z);
            major.push(end_z);
        } else if i % SPACING[1] == 0 {
            medium.push(start_x);
            medium.push(end_x);
            medium.push(start_z);
            medium.push(end_z);
        } else {
            minor.push(start_x);
            minor.push(end_x);
            minor.push(start_z);
            minor.push(end_z);
        }
    }
    (minor, medium, major)
}

fn create_mesh(meshes: &mut ResMut<Assets<Mesh>>, vertices: Vec<Vec3>) -> Mesh3d {
    let mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    Mesh3d(meshes.add(mesh))
}
