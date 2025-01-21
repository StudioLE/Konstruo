use bevy::math::Vec3;
use Orientation::*;

/// Orientation terminology according to the six sides of a cuboid.
/// 
/// A face is defined by a single orientation.
///
/// An edge is defined by two orientations.
///
/// A corner is defined by three orientations. 
#[derive(Clone, Debug)]
pub enum Orientation {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

impl Orientation {
    /// Get all [`Orientation`]
    pub fn get_all() -> [Orientation; 6] {
        [Front, Back, Left, Right, Top, Bottom]
    }

    /// Get the vector facing in the orientation
    pub fn to_vector(&self) -> Vec3 {
        match self {
            Front => Vec3::NEG_Y,
            Back => Vec3::Y,
            Left => Vec3::NEG_X,
            Right => Vec3::X,
            Top => Vec3::Z,
            Bottom => Vec3::NEG_Z,
        }
    }

    /// Get the vector facing in the combined orientation
    pub fn get_vector(orientation: &[Orientation]) -> Vec3 {
        orientation
            .iter()
            .fold(Vec3::ZERO, |acc, orientation| acc + orientation.to_vector())
    }
}
