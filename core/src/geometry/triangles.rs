use bevy::math::Vec3;

#[allow(clippy::indexing_slicing)]
/// Add vertices to a polyline by splitting the longest edge.
pub fn add_vertices_by_spliting_longest_edge(short: &mut Vec<Vec3>, count: usize) {
    for _ in 0..count {
        let starts = short.iter().take(short.len() - 1);
        let ends = short.iter().skip(1);
        let distances = starts
            .zip(ends)
            .map(|(start, end)| start.distance(*end))
            .collect::<Vec<f32>>();
        let max_distance = distances
            .iter()
            .max_by(|a, b| a.partial_cmp(b).expect("Floats are comparable"))
            .expect("Should be at least one distance");
        #[allow(clippy::float_cmp)]
        let max_index = distances
            .iter()
            .position(|&x| x == *max_distance)
            .expect("Item should exist");
        let mid_point = (short[max_index] + short[max_index + 1]) / 2.0;
        short.insert(max_index + 1, mid_point);
    }
}
