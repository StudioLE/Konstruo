use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;

/// Extract the primitive topology from vertices.
pub fn extract_topology(vertices: Vec<Vec3>, topology: PrimitiveTopology) -> Vec<Vec<Vec3>> {
    match topology {
        PrimitiveTopology::PointList => point_list(vertices),
        PrimitiveTopology::LineList => line_list(vertices),
        PrimitiveTopology::LineStrip => vec![vertices],
        PrimitiveTopology::TriangleList => triangle_list(vertices),
        PrimitiveTopology::TriangleStrip => triangle_strip(vertices),
    }
}

/// Create a [`PrimitiveTopology::TriangleStrip`] between from two parallel polylines.
///
/// The polylines must have equal number of vertices.
pub fn create_triangle_strip_between_polylines(polylines: &[Vec<Vec3>; 2]) -> Vec<Vec3> {
    let count_0 = polylines[0].len();
    let count_1 = polylines[1].len();
    assert_eq!(count_0, count_1, "Failed to create triangle strip between polylines. Vertices count must be equal. {count_0} != {count_1}");
    polylines[0]
        .iter()
        .zip(polylines[1].iter())
        .flat_map(|(&a, &b)| [a, b])
        .collect()
}

fn point_list(vertices: Vec<Vec3>) -> Vec<Vec<Vec3>> {
    let mut points = Vec::new();
    for vertex in vertices {
        points.push(vec![vertex]);
    }
    points
}

#[allow(clippy::indexing_slicing)]
fn line_list(vertices: Vec<Vec3>) -> Vec<Vec<Vec3>> {
    let count = vertices.len();
    if count < 2 {
        panic!("Failed to extract lines. Vertices count must be at least 2.")
    } else if count % 2 != 0 {
        panic!("Failed to extract lines. Vertices count must be an even number.")
    }
    let mut lines = Vec::new();
    for i in 1..count {
        let start = vertices[i - 1];
        let end = vertices[i];
        lines.push(vec![start, end]);
    }
    lines
}

#[allow(clippy::indexing_slicing)]
fn triangle_list(vertices: Vec<Vec3>) -> Vec<Vec<Vec3>> {
    let count = vertices.len();
    if count < 3 {
        panic!("Failed to extract triangles. Vertices count must be at least 3.")
    } else if count % 3 != 0 {
        panic!("Failed to extract triangles. Vertices count must be a multiple of 3.")
    }
    #[allow(clippy::integer_division)]
    let count = count / 3;
    let mut triangles = Vec::new();
    for i in 0..count {
        let i = i * 3;
        let a = vertices[i];
        let b = vertices[i + 1];
        let c = vertices[i + 2];
        triangles.push(vec![a, b, c]);
    }
    triangles
}

#[allow(clippy::indexing_slicing)]
fn triangle_strip(vertices: Vec<Vec3>) -> Vec<Vec<Vec3>> {
    let count = vertices.len();
    if count < 3 {
        panic!("Failed to extract triangles. Vertices count must be at least 3.")
    } else if count % 3 != 0 {
        panic!("Failed to extract triangles. Vertices count must be a multiple of 3.")
    }
    let mut triangles = Vec::new();
    for i in 2..count {
        let a = vertices[i - 2];
        let b = vertices[i - 1];
        let c = vertices[i];
        triangles.push(vec![a, b, c]);
    }
    triangles
}
