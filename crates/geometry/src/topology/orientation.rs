use bevy::math::Vec3;
use konstruo_core::{Vec3Extensions, HALF_PI, PI};
use std::fmt::{Display, Formatter};
use std::ops::Neg;
use Orientation::*;

/// Orientation terminology according to the six sides of a cuboid.
///
/// A face is defined by a single orientation.
///
/// An edge is defined by two orientations.
///
/// A corner is defined by three orientations.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Orientation {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

impl Display for Orientation {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Front => "Front",
            Back => "Back",
            Left => "Left",
            Right => "Right",
            Top => "Top",
            Bottom => "Bottom",
        };
        formatter.write_str(name)
    }
}

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
    pub fn to_facing_in(&self) -> Vec3 {
        match self {
            Front => Vec3::NEG_Y,
            Back => Vec3::Y,
            Left => Vec3::NEG_X,
            Right => Vec3::X,
            Top => Vec3::Z,
            Bottom => Vec3::NEG_Z,
        }
    }

    /// Get the vector facing towards the elevation with the orientation name.
    #[must_use]
    pub fn to_facing_to(&self) -> Vec3 {
        self.to_facing_in().neg()
    }

    /// Get the vector facing in the combined orientation.
    ///
    /// This is the opposite of the vector looking at an elevation with the orientation name.
    #[must_use]
    pub fn get_facing_in(orientation: &[Orientation]) -> Vec3 {
        orientation.iter().fold(Vec3::ZERO, |acc, orientation| {
            acc + orientation.to_facing_in()
        })
    }

    /// Get the axis when looking at the elevation.
    #[must_use]
    pub fn to_elevation_axis(&self) -> (Vec3, Vec3, Vec3) {
        let back = self.to_facing_to();
        let up = if Vec3Extensions::is_almost_equal_to(back, Vec3::Z) {
            Vec3::NEG_Y
        } else if Vec3Extensions::is_almost_equal_to(back, Vec3::NEG_Z) {
            Vec3::Y
        } else {
            Vec3::Z
        };
        let right = back.cross(up).normalize();
        (right, up, back)
    }

    /// Get the z rotation of the orientation.
    #[must_use]
    pub fn get_z_rotation(&self) -> f32 {
        match self {
            Left => -HALF_PI,
            Right => HALF_PI,
            Back => PI,
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_elevation_axis() {
        let (right, up, back) = Front.to_elevation_axis();
        assert_eq!(right, Vec3::X);
        assert_eq!(up, Vec3::Z);
        assert_eq!(back, Vec3::Y);
        let (right, up, back) = Right.to_elevation_axis();
        assert_eq!(right, Vec3::Y);
        assert_eq!(up, Vec3::Z);
        assert_eq!(back, Vec3::NEG_X);
        let (right, up, back) = Back.to_elevation_axis();
        assert_eq!(right, Vec3::NEG_X);
        assert_eq!(up, Vec3::Z);
        assert_eq!(back, Vec3::NEG_Y);
        let (right, up, back) = Left.to_elevation_axis();
        assert_eq!(right, Vec3::NEG_Y);
        assert_eq!(up, Vec3::Z);
        assert_eq!(back, Vec3::X);
        let (right, up, back) = Top.to_elevation_axis();
        assert_eq!(right, Vec3::X);
        assert_eq!(up, Vec3::Y);
        assert_eq!(back, Vec3::NEG_Z);
        let (right, up, back) = Bottom.to_elevation_axis();
        assert_eq!(right, Vec3::X);
        assert_eq!(up, Vec3::NEG_Y);
        assert_eq!(back, Vec3::Z);
    }
}
