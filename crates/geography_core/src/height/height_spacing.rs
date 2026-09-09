//! Interval between height vertices.

use crate::CHUNK_SIZE;

/// Interval between height vertices, in meters.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HeightSpacing {
    /// Two meters.
    Two,
    /// Four meters.
    Four,
    /// Eight meters.
    Eight,
    /// Sixteen meters.
    Sixteen,
}

impl HeightSpacing {
    /// Every spacing generated, finest first.
    pub const ALL: [Self; 4] = [Self::Two, Self::Four, Self::Eight, Self::Sixteen];

    /// Interval between vertices, in meters.
    #[must_use]
    pub const fn meters(self) -> i32 {
        match self {
            Self::Two => 2,
            Self::Four => 4,
            Self::Eight => 8,
            Self::Sixteen => 16,
        }
    }

    /// Number of vertices along one side of a chunk.
    ///
    /// - Spans the chunk, plus one border vertex beyond each side
    #[expect(
        clippy::as_conversions,
        clippy::cast_sign_loss,
        clippy::integer_division,
        reason = "exact positive division"
    )]
    #[must_use]
    pub const fn vertices_across(self) -> usize {
        (CHUNK_SIZE / self.meters() + 3) as usize
    }

    /// Name of the directory holding chunks at this spacing.
    #[must_use]
    pub const fn directory(self) -> &'static str {
        match self {
            Self::Two => "2m",
            Self::Four => "4m",
            Self::Eight => "8m",
            Self::Sixteen => "16m",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_meters() {
        // Act
        let meters: Vec<i32> = HeightSpacing::ALL
            .iter()
            .map(|spacing| spacing.meters())
            .collect();
        // Assert
        assert_eq!(meters, vec![2, 4, 8, 16]);
    }

    #[test]
    fn spacing_vertices_across() {
        // Act
        let across: Vec<usize> = HeightSpacing::ALL
            .iter()
            .map(|spacing| spacing.vertices_across())
            .collect();
        // Assert
        assert_eq!(across, vec![259, 131, 67, 35]);
    }

    #[test]
    fn spacing_directory() {
        // Act
        let directories: Vec<&str> = HeightSpacing::ALL
            .iter()
            .map(|spacing| spacing.directory())
            .collect();
        // Assert
        assert_eq!(directories, vec!["2m", "4m", "8m", "16m"]);
    }
}
