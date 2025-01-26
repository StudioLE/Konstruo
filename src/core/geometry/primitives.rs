use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use std::fmt::{Display, Formatter};
use ExtractTopologyError::*;

/// Extract the primitive topology from vertices.
pub fn extract_topology(
    vertices: Vec<Vec3>,
    topology: PrimitiveTopology,
) -> Result<Vec<Vec<Vec3>>, ExtractTopologyError> {
    match topology {
        PrimitiveTopology::PointList => Ok(point_list(vertices)),
        PrimitiveTopology::LineList => line_list(vertices),
        PrimitiveTopology::LineStrip => Ok(vec![vertices]),
        PrimitiveTopology::TriangleList => triangle_list(vertices),
        PrimitiveTopology::TriangleStrip => triangle_strip(vertices),
    }
}

fn point_list(vertices: Vec<Vec3>) -> Vec<Vec<Vec3>> {
    let mut points = Vec::new();
    for vertex in vertices {
        points.push(vec![vertex]);
    }
    points
}

#[allow(clippy::indexing_slicing)]
fn line_list(vertices: Vec<Vec3>) -> Result<Vec<Vec<Vec3>>, ExtractTopologyError> {
    let count = vertices.len();
    if count < 2 {
        return Err(VerticesMin { count, min: 2 });
    } else if count % 2 != 0 {
        return Err(VerticesDivision { count, divisor: 2 });
    }
    let mut lines = Vec::new();
    for i in 1..count {
        let start = vertices[i - 1];
        let end = vertices[i];
        lines.push(vec![start, end]);
    }
    Ok(lines)
}

#[allow(clippy::indexing_slicing)]
fn triangle_list(vertices: Vec<Vec3>) -> Result<Vec<Vec<Vec3>>, ExtractTopologyError> {
    let count = vertices.len();
    if count < 3 {
        return Err(VerticesMin { count, min: 3 });
    } else if count % 3 != 0 {
        return Err(VerticesDivision { count, divisor: 3 });
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
    Ok(triangles)
}

#[allow(clippy::indexing_slicing)]
fn triangle_strip(vertices: Vec<Vec3>) -> Result<Vec<Vec<Vec3>>, ExtractTopologyError> {
    let count = vertices.len();
    if count < 3 {
        return Err(VerticesMin { count, min: 3 });
    } else if count % 3 != 0 {
        return Err(VerticesDivision { count, divisor: 3 });
    }
    let mut triangles = Vec::new();
    for i in 2..count {
        let a = vertices[i - 2];
        let b = vertices[i - 1];
        let c = vertices[i];
        triangles.push(vec![a, b, c]);
    }
    Ok(triangles)
}

#[derive(Debug, PartialEq)]
pub enum ExtractTopologyError {
    VerticesMin { count: usize, min: usize },
    VerticesDivision { count: usize, divisor: usize },
}

impl Display for ExtractTopologyError {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            VerticesMin { count, min } => {
                format!("Vertices count must be at least {min}: {count}")
            }
            VerticesDivision { count, divisor } => {
                format!("Vertices count must be divisible by {divisor}: {count}")
            }
        };
        output.fmt(formatter)
    }
}