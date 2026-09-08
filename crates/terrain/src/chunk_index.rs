//! Position of a chunk within the chunk grid.

use crate::chunk_bounds::CHUNK_SIZE;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use studiole_report::prelude::*;
use thiserror::Error;

/// Position of a chunk within the chunk grid.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChunkIndex {
    /// Index along X, increasing east.
    pub x: i32,
    /// Index along Y, increasing north.
    pub y: i32,
}

impl ChunkIndex {
    /// Create a new [`ChunkIndex`].
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Chunk containing a world position, in meters.
    ///
    /// - Floors, so positions west or south of the origin land in the correct chunk
    #[expect(
        clippy::as_conversions,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        reason = "world extent is far within i32"
    )]
    #[must_use]
    pub fn from_world(x: f32, y: f32) -> Self {
        let size = CHUNK_SIZE as f32;
        Self {
            x: x.div_euclid(size) as i32,
            y: y.div_euclid(size) as i32,
        }
    }
}

impl Display for ChunkIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let east_west = if self.x < 0 { 'W' } else { 'E' };
        let north_south = if self.y < 0 { 'S' } else { 'N' };
        let x = self.x.unsigned_abs();
        let y = self.y.unsigned_abs();
        write!(f, "{east_west}{x:02}{north_south}{y:02}")
    }
}

impl FromStr for ChunkIndex {
    type Err = Report<ChunkIndexError>;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let mut characters = name.chars();
        let east_west = characters
            .next()
            .ok_or_else(|| Report::new(ChunkIndexError::EastWest))?;
        let remainder = characters.as_str();
        let at = remainder
            .find(['N', 'S'])
            .ok_or_else(|| Report::new(ChunkIndexError::NorthSouth))?;
        let (x, northing) = remainder.split_at(at);
        let mut characters = northing.chars();
        let north_south = characters
            .next()
            .ok_or_else(|| Report::new(ChunkIndexError::NorthSouth))?;
        let y = characters.as_str();
        Ok(Self {
            x: parse_axis(east_west, x, 'E', 'W', ChunkIndexError::EastWest)?,
            y: parse_axis(north_south, y, 'N', 'S', ChunkIndexError::NorthSouth)?,
        })
    }
}

/// Read one axis of a chunk name.
///
/// - The letter carries the sign, so there is no `W00` or `S00`
fn parse_axis(
    letter: char,
    digits: &str,
    positive: char,
    negative: char,
    missing: ChunkIndexError,
) -> Result<i32, Report<ChunkIndexError>> {
    if letter != positive && letter != negative {
        return Err(Report::new(missing).attach("letter", letter));
    }
    if !is_padded_digits(digits) {
        return Err(Report::new(ChunkIndexError::Digits).attach("digits", digits));
    }
    let value = i32::from_str(digits)
        .change_context(ChunkIndexError::Digits)
        .attach("digits", digits)?;
    if letter == positive {
        Ok(value)
    } else if value == 0 {
        Err(Report::new(ChunkIndexError::SignedZero).attach("letter", letter))
    } else {
        Ok(-value)
    }
}

/// Is a digit run zero padded to the width [`Display`] writes?
fn is_padded_digits(digits: &str) -> bool {
    let is_digits = !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit());
    let is_padded = digits.len() == 2 || (digits.len() > 2 && !digits.starts_with('0'));
    is_digits && is_padded
}

/// Errors returned by [`ChunkIndex`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ChunkIndexError {
    /// Missing east or west prefix.
    #[error("Missing east or west prefix")]
    EastWest,
    /// Missing north or south delimiter.
    #[error("Missing north or south delimiter")]
    NorthSouth,
    /// Malformed digits.
    #[error("Malformed digits")]
    Digits,
    /// Zero is neither west nor south.
    #[error("Zero is neither west nor south")]
    SignedZero,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Indices covering the origin, both signs, and both corners of the union.
    const INDICES: [ChunkIndex; 6] = [
        ChunkIndex::new(0, 0),
        ChunkIndex::new(-1, 0),
        ChunkIndex::new(0, -1),
        ChunkIndex::new(-1, -1),
        ChunkIndex::new(29, 29),
        ChunkIndex::new(-30, -40),
    ];

    #[test]
    fn chunk_index_display() {
        // Act
        let display: Vec<String> = INDICES.iter().map(ToString::to_string).collect();
        // Assert
        assert_eq!(
            display,
            vec!["E00N00", "W01N00", "E00S01", "W01S01", "E29N29", "W30S40"]
        );
    }

    /// [`FromStr`] is the exact inverse of [`Display`].
    #[test]
    fn chunk_index_from_str() {
        // Act
        let parsed: Vec<Option<ChunkIndex>> = INDICES
            .iter()
            .map(|index| ChunkIndex::from_str(&index.to_string()).ok())
            .collect();
        // Assert
        let expected: Vec<Option<ChunkIndex>> = INDICES.iter().copied().map(Some).collect();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn chunk_index_from_str_malformed() {
        // Arrange
        let names = ["W00N00", "E00S00", "E0N00", "X00N00", "E00"];
        // Act
        let parsed: Vec<Option<ChunkIndex>> = names
            .iter()
            .map(|name| ChunkIndex::from_str(name).ok())
            .collect();
        // Assert
        assert_eq!(parsed, vec![None; 5]);
    }

    /// A truncating division would give `0` for the negative case.
    #[test]
    fn chunk_index_from_world() {
        // Act
        let origin = ChunkIndex::from_world(0.0, 0.0);
        let west = ChunkIndex::from_world(-512.0, -1.0);
        let inside = ChunkIndex::from_world(511.9, 0.1);
        let north = ChunkIndex::from_world(512.0, 512.0);
        // Assert
        assert_eq!(origin, ChunkIndex::new(0, 0));
        assert_eq!(west, ChunkIndex::new(-1, -1));
        assert_eq!(inside, ChunkIndex::new(0, 0));
        assert_eq!(north, ChunkIndex::new(1, 1));
    }
}
