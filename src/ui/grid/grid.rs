use super::*;
use crate::geometry::LineList;
use crate::{GRID_ELEVATION, GRID_MAX};
use bevy::prelude::*;

#[allow(clippy::integer_division)]
const RADIUS: u32 = GRID_MAX / 2;
const SPACING: [u32; 3] = [1, 10, 100];

#[allow(clippy::integer_division)]
const COUNT: u32 = (RADIUS * 2) / SPACING[0] + 1;

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
        let [minor, medium, major] = create_lines();
        let minor = (
            Grid::Minor,
            Mesh3d(meshes.add(minor.to_mesh())),
            MeshMaterial3d(materials.minor.clone()),
        );
        let medium = (
            Grid::Medium,
            Mesh3d(meshes.add(medium.to_mesh())),
            MeshMaterial3d(materials.medium.clone()),
        );
        let major = (
            Grid::Major,
            Mesh3d(meshes.add(major.to_mesh())),
            MeshMaterial3d(materials.major.clone()),
        );
        commands.spawn(minor);
        commands.spawn(medium);
        commands.spawn(major);
    }
}

#[allow(clippy::as_conversions, clippy::cast_precision_loss)]
fn create_lines() -> [LineList; 3] {
    let range = 0..COUNT;
    let radius = RADIUS as f32;
    let mut minor = Vec::new();
    let mut medium = Vec::new();
    let mut major = Vec::new();
    for i in range {
        let a = i as f32 - radius;
        let b = (radius.powi(2) - a.powi(2)).sqrt();
        let start_x = Vec3::new(a, b, GRID_ELEVATION);
        let end_x = Vec3::new(a, -b, GRID_ELEVATION);
        let start_y = Vec3::new(-b, a, GRID_ELEVATION);
        let end_y = Vec3::new(b, a, GRID_ELEVATION);
        if i % SPACING[2] == 0 {
            major.push([start_x, end_x]);
            major.push([start_y, end_y]);
        } else if i % SPACING[1] == 0 {
            medium.push([start_x, end_x]);
            medium.push([start_y, end_y]);
        } else {
            minor.push([start_x, end_x]);
            minor.push([start_y, end_y]);
        }
    }
    [
        LineList::new(minor),
        LineList::new(medium),
        LineList::new(major),
    ]
}
