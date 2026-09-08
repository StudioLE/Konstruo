//! Track how many source files each chunk is still owed.

use crate::prelude::*;

/// Track how many source files each chunk is still owed.
///
/// - Counting up front and decrementing lets a chunk be written and freed as
///   soon as its last contributing file lands, whatever order files arrive in
#[derive(Default)]
pub struct ChunkCompletion {
    /// Number of source files each chunk is still owed.
    remaining: HashMap<ChunkIndex, usize>,
}

impl ChunkCompletion {
    /// Create a new [`ChunkCompletion`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record one source file contributing to every chunk within bounds.
    pub fn add(&mut self, bounds: ChunkBounds) {
        for index in bounds.indices() {
            *self.remaining.entry(index).or_insert(0) += 1;
        }
    }

    /// Record one source file landing on a chunk.
    ///
    /// # Panics
    ///
    /// - Panics IF the chunk was never counted by [`ChunkCompletion::add`]
    /// - Panics IF more files land on the chunk than were counted
    pub fn record(&mut self, index: ChunkIndex) {
        let remaining = self
            .remaining
            .get_mut(&index)
            .expect("chunk should be counted before a file lands on it");
        *remaining = remaining
            .checked_sub(1)
            .expect("chunk should not receive more files than were counted");
    }

    /// Is a chunk owed no further source files?
    #[must_use]
    pub fn is_complete(&self, index: ChunkIndex) -> bool {
        self.remaining.get(&index) == Some(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A chunk two files overlap completes only when the second lands.
    #[test]
    fn chunk_completion_is_complete() {
        // Arrange
        let index = ChunkIndex::new(0, 0);
        let mut completion = ChunkCompletion::new();
        completion.add(ChunkBounds::new(0, 1, 0, 1));
        completion.add(ChunkBounds::new(-1, 0, -1, 0));
        // Act
        completion.record(index);
        let first = completion.is_complete(index);
        completion.record(index);
        let second = completion.is_complete(index);
        // Assert
        assert!(!first);
        assert!(second);
    }

    /// A chunk one file overlaps completes when that file lands.
    #[test]
    fn chunk_completion_is_complete_once() {
        // Arrange
        let index = ChunkIndex::new(1, 1);
        let mut completion = ChunkCompletion::new();
        completion.add(ChunkBounds::new(0, 1, 0, 1));
        // Act
        completion.record(index);
        // Assert
        assert!(completion.is_complete(index));
    }

    /// A chunk no file was counted against is a bug, not a silent skip.
    #[test]
    #[should_panic(expected = "chunk should be counted before a file lands on it")]
    fn chunk_completion_record_uncounted() {
        let mut completion = ChunkCompletion::new();
        completion.record(ChunkIndex::new(0, 0));
    }
}
