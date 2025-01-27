use super::*;
use crate::beziers::CubicBezierSpline;
use crate::geometry::{Polyline, TriangleList};
use crate::GROUND_HEIGHT;
use bevy::prelude::*;
use SurfaceType::*;

/// A surface formed by two lines from a [Way].
#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WaySurface {
    /// Depth of the surface.
    depth: f32,
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
    pub fn new(depth: f32, offsets: [f32; 2], purpose: SurfaceType) -> Self {
        Self {
            depth,
            offsets,
            purpose,
        }
    }

    /// Create a new [`WaySurface`] centered at [`Way`].
    #[must_use]
    pub fn centered(depth: f32, width: f32, purpose: SurfaceType) -> Self {
        Self::new(depth, [width * -0.5, width * 0.5], purpose)
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
        let bundle = (
            self,
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::new(0.0, 0.0, GROUND_HEIGHT)),
        );
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
    fn get_polylines(&self, way: &Way) -> [Polyline; 2] {
        let splines = self.get_splines(way);
        [
            splines[0].flatten(FLATTEN_TOLERANCE).into(),
            splines[1].flatten(FLATTEN_TOLERANCE).into(),
        ]
    }

    /// Get the [`Mesh`].
    fn get_mesh(&self, way: &Way) -> Mesh {
        let [bottom_0, bottom_1] = self.get_polylines(way).map(Polyline::to_vertices);
        let top_0: Vec<Vec3> = bottom_0
            .iter()
            .map(|vertex| vertex.with_z(vertex.z + self.depth))
            .collect();
        let top_1: Vec<Vec3> = bottom_1
            .iter()
            .map(|vertex| vertex.with_z(vertex.z + self.depth))
            .collect();
        let start_top = vec![
            *top_0.first().expect("first should exist"),
            *top_1.first().expect("first should exist"),
        ];

        let start_bottom = vec![
            *bottom_0.first().expect("first should exist"),
            *bottom_1.first().expect("first should exist"),
        ];
        let end_top = vec![
            *top_0.last().expect("last should exist"),
            *top_1.last().expect("last should exist"),
        ];
        let end_bottom = vec![
            *bottom_0.last().expect("first should exist"),
            *bottom_1.last().expect("last should exist"),
        ];

        let mut triangles =
            TriangleList::between_polylines([top_0.clone().into(), top_1.clone().into()]);
        triangles.merge(TriangleList::between_polylines([
            bottom_0.clone().into(),
            bottom_1.clone().into(),
        ]));
        triangles.merge(TriangleList::between_polylines([
            top_0.into(),
            bottom_0.into(),
        ]));
        triangles.merge(TriangleList::between_polylines([
            top_1.into(),
            bottom_1.into(),
        ]));
        triangles.merge(TriangleList::between_polylines([
            start_top.into(),
            start_bottom.into(),
        ]));
        triangles.merge(TriangleList::between_polylines([
            end_top.into(),
            end_bottom.into(),
        ]));
        triangles.to_mesh()
    }
}
