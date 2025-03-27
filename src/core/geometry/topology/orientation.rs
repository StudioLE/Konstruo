use bevy::math::Vec3;
use Orientation::*;

/// Orientation terminology according to the six sides of a cuboid.
///
/// A face is defined by a single orientation.
///
/// An edge is defined by two orientations.
///
/// A corner is defined by three orientations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Orientation {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

impl Orientation {}

impl Orientation {
    /// Get all [`Orientation`]
    #[must_use]
    pub fn get_all() -> [Orientation; 6] {
        [Front, Back, Left, Right, Top, Bottom]
    }

    /// Get all edges as [`Orientation`] pairs.
    #[must_use]
    pub fn get_all_edges() -> [[Orientation; 2]; 12] {
        [
            [Front, Left],
            [Front, Right],
            [Front, Top],
            [Front, Bottom],
            [Back, Left],
            [Back, Right],
            [Back, Top],
            [Back, Bottom],
            [Left, Top],
            [Left, Bottom],
            [Right, Top],
            [Right, Bottom],
        ]
    }

    /// Get all corners as [`Orientation`] triplets.
    #[must_use]
    pub fn get_all_corners() -> [[Orientation; 3]; 8] {
        [
            [Front, Left, Top],
            [Front, Right, Top],
            [Front, Left, Bottom],
            [Front, Right, Bottom],
            [Back, Left, Top],
            [Back, Right, Top],
            [Back, Left, Bottom],
            [Back, Right, Bottom],
        ]
    }

    /// Get the vector facing in the orientation.
    ///
    /// This is the opposite of the vector looking at an elevation with the orientation name.
    #[must_use]
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
    #[must_use]
    pub fn get_vector(orientation: &[Orientation]) -> Vec3 {
        orientation
            .iter()
            .fold(Vec3::ZERO, |acc, orientation| acc + orientation.to_vector())
    }
}
