use super::*;
use crate::beziers::CubicBezierSpline;
use crate::geometry::TriangleList;
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
    ///
    /// The polylines will have the same number of vertices.
    fn get_polylines(&self, way: &Way) -> [Vec<Vec3>; 2] {
        let splines = self.get_splines(way);
        [
            splines[0].flatten(FLATTEN_TOLERANCE),
            splines[1].flatten(FLATTEN_TOLERANCE),
        ]
    }

    /// Get the [`Mesh`].
    fn get_mesh(&self, way: &Way) -> Mesh {
        let bottom = self.get_polylines(way);
        let top: [Vec<Vec3>; 2] = [
            bottom[0]
                .iter()
                .map(|vertex| vertex.with_z(vertex.z + self.depth))
                .collect(),
            bottom[1]
                .iter()
                .map(|vertex| vertex.with_z(vertex.z + self.depth))
                .collect(),
        ];
        let ends: [[Vec<Vec3>; 2]; 2] = [
            [
                vec![
                    *top[0].first().expect("first should exist"),
                    *top[1].first().expect("first should exist"),
                ],
                vec![
                    *bottom[0].first().expect("first should exist"),
                    *bottom[1].first().expect("first should exist"),
                ],
            ],
            [
                vec![
                    *top[0].last().expect("last should exist"),
                    *top[1].last().expect("last should exist"),
                ],
                vec![
                    *bottom[0].last().expect("first should exist"),
                    *bottom[1].last().expect("last should exist"),
                ],
            ],
        ];
        let mut triangles = TriangleList::between_polylines(&top);
        triangles.merge(TriangleList::between_polylines(&bottom));
        triangles.merge(TriangleList::between_polylines(&[
            top[0].clone(),
            bottom[0].clone(),
        ]));
        triangles.merge(TriangleList::between_polylines(&[
            top[1].clone(),
            bottom[1].clone(),
        ]));
        triangles.merge(TriangleList::between_polylines(&ends[0]));
        triangles.merge(TriangleList::between_polylines(&ends[1]));
        triangles.to_mesh()
    }
}
