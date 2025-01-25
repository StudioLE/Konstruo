use crate::core::geometry::meshes::create_linestrip;
use crate::infrastructure::ways::{Way, WayMaterials};
use bevy::prelude::*;

/// A line between control points of a [`Way`].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WayControlLine {
    /// Index of the anchor in the spline of the [`Way`].
    pub anchor: usize,
    /// Index of the handle in the spline of the [`Way`].
    pub handle: usize,
}
impl WayControlLine {
    /// Create a new [`WayControlLine`]
    fn new(start: usize, end: usize) -> Self {
        Self {
            anchor: start,
            handle: end,
        }
    }

    /// Factory method to spawn [`WayControlLine`] for each control point in a [`Way`]
    pub(super) fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &Res<WayMaterials>,
        way: &Way,
        parent: Entity,
    ) {
        for (i, bezier) in way.spline.curves.iter().enumerate() {
            let i = i * 4;
            let start = (
                WayControlLine::new(i, i + 1),
                Mesh3d(meshes.add(create_linestrip(vec![bezier.start, bezier.start_handle]))),
                MeshMaterial3d(materials.control_line.clone()),
            );
            let end = (
                WayControlLine::new(i + 3, i + 2),
                Mesh3d(meshes.add(create_linestrip(vec![bezier.end, bezier.end_handle]))),
                MeshMaterial3d(materials.control_line.clone()),
            );
            commands.spawn(start).set_parent(parent);
            commands.spawn(end).set_parent(parent);
        }
    }
}
