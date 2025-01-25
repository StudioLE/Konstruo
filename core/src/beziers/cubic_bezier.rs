use bevy::prelude::*;

/// A single cubic bezier curve of four control points.
#[derive(Clone, Debug, Default)]
pub struct CubicBezier {
    pub start: Vec3,
    pub start_handle: Vec3,
    pub end_handle: Vec3,
    pub end: Vec3,
}

impl CubicBezier {
    /// Get the four controls.
    #[must_use]
    pub fn get_controls(&self) -> [Vec3; 4] {
        [self.start, self.start_handle, self.end_handle, self.end]
    }
}
