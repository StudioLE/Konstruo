use crate::core::beziers::CubicBezierSpline;
use crate::core::geometry::triangles::add_vertices_by_spliting_longest_edge;
use crate::core::geometry::TriangleStip;
use crate::infrastructure::ways::way::Way;
use crate::infrastructure::ways::{WayMaterials, FLATTEN_TOLERANCE, OFFSET_ACCURACY};
use bevy::prelude::*;
use std::cmp::Ordering;
use SurfaceType::*;

/// A surface formed by two lines from a [Way].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WaySurface {
    /// Offsets from the way.
    offsets: [f32; 2],

    /// Offsets from the way.
    purpose: SurfaceType,
}

pub enum SurfaceType {
    /// - <https://en.wikipedia.org/wiki/Carriageway>
    Carriageway,
    /// - <https://en.wikipedia.org/wiki/Footway>
    Footway,
    /// - <https://en.wikipedia.org/wiki/Road_verge>
    Verge,
}

impl WaySurface {
    /// Create a new [`WaySurface`] offset from [`Way`].
    #[must_use]
    pub fn new(offsets: [f32; 2], purpose: SurfaceType) -> Self {
        Self { offsets, purpose }
    }

    /// Create a new [`WaySurface`] centered at [`Way`].
    #[must_use]
    pub fn centered(width: f32, purpose: SurfaceType) -> Self {
        Self::new([width * -0.5, width * 0.5], purpose)
    }

    /// Regenerate the mesh geometry.
    pub fn regenerate(&self, meshes: &mut ResMut<Assets<Mesh>>, mut mesh: Mut<Mesh3d>, way: &Way) {
        *mesh = Mesh3d(meshes.add(self.get_mesh(way)));
    }

    /// Spawn a [`WaySurface`] with its mesh geometry.
    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &Res<WayMaterials>,
        way: &Way,
        parent: Entity,
    ) {
        let mesh = self.get_mesh(way);
        let material = match self.purpose {
            Carriageway => materials.carriageway.clone(),
            Footway => materials.footway.clone(),
            Verge => materials.verge.clone(),
        };
        let bundle = (self, Mesh3d(meshes.add(mesh)), MeshMaterial3d(material));
        commands.spawn(bundle).set_parent(parent);
    }

    /// Get the splines of each edge.
    fn get_splines(&self, way: &Way) -> [CubicBezierSpline; 2] {
        [
            way.spline.offset(self.offsets[0], OFFSET_ACCURACY),
            way.spline.offset(self.offsets[1], OFFSET_ACCURACY),
        ]
    }

    /// Get the polylines of each edge.
    ///
    /// The polylines will have the same number of vertices.
    #[allow(
        clippy::as_conversions,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    fn get_polylines(&self, way: &Way) -> [Vec<Vec3>; 2] {
        let splines = self.get_splines(way);
        let mut polylines = [
            splines[0].flatten(FLATTEN_TOLERANCE),
            splines[1].flatten(FLATTEN_TOLERANCE),
        ];
        let difference = polylines[0].len() as isize - polylines[1].len() as isize;
        match difference.cmp(&0) {
            Ordering::Less => {
                add_vertices_by_spliting_longest_edge(&mut polylines[0], difference.unsigned_abs());
            }
            Ordering::Greater => {
                add_vertices_by_spliting_longest_edge(&mut polylines[1], difference as usize);
            }
            Ordering::Equal => {}
        }
        polylines
    }

    /// Get the [`Mesh`].
    fn get_mesh(&self, way: &Way) -> Mesh {
        let polylines = self.get_polylines(way);
        TriangleStip::between_polylines(&polylines)
            .expect("polylines should have equal length")
            .to_mesh()
    }
}
