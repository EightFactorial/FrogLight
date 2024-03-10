use froglight_core::common::ChunkBlockPosition;

use super::Chunk;
use crate::world::Section;

/// An iterator over all of the blocks in a [`Chunk`].
///
/// # Example
/// ```rust
/// use froglight_world::world::Chunk;
///
/// // Create a new empty chunk and get an iterator over its blocks.
/// // Using the shape of an Overworld Chunk for the example.
/// let chunk = Chunk::new_empty(320, -64);
///
/// // Create an iterator.
/// let mut iterator = chunk.block_iter();
///
/// // Check that all values are 0.
/// while let Some(block) = iterator.next() {
///     assert_eq!(block, 0);
/// }
///
/// // Check that the iterator is finished.
/// assert!(iterator.is_finished());
/// assert_eq!(iterator.next(), None);
/// ```
#[derive(Debug)]
pub struct ChunkIdIterator<'c> {
    chunk: &'c Chunk,
    position: ChunkBlockPosition,
    finished: bool,
}

impl<'c> ChunkIdIterator<'c> {
    /// Creates a new [`ChunkIdIterator`] for the given [`Chunk`].
    #[must_use]
    pub const fn new(chunk: &'c Chunk) -> Self {
        Self { chunk, position: ChunkBlockPosition::ZERO, finished: false }
    }

    /// Creates a new [`ChunkIdIterator`] for the given [`Chunk`] starting at
    /// the given position.
    #[must_use]
    pub const fn starting_at(chunk: &'c Chunk, position: ChunkBlockPosition) -> Self {
        Self { chunk, position, finished: false }
    }

    /// Gets the current position of the iterator.
    #[must_use]
    pub const fn position(&self) -> ChunkBlockPosition { self.position }

    /// Returns whether the iterator is finished.
    #[must_use]
    pub const fn is_finished(&self) -> bool { self.finished }
}

impl Iterator for ChunkIdIterator<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // If the iterator is finished, return `None`.
        if self.finished {
            return None;
        }

        // Get the block id at the current position.
        let result = self.chunk.get_blockid(&self.position);

        // If the position is at the end, mark the iterator as finished.
        if usize::from(self.position.x) == Section::WIDTH - 1
            && usize::from(self.position.z) == Section::DEPTH - 1
            && self.position.y == self.chunk.height - 1
        {
            self.finished = true;
        } else if result.is_some() {
            self.position = self.position.next();
        }

        result
    }
}

#[test]
fn chunkid_iter() {
    // Create a new empty chunk and get an iterator over its blocks.
    let chunk = Chunk::new_empty(320, -64);
    let mut iter = chunk.block_iter();

    // Check that all values are 0.
    for y in 0..chunk.height {
        for z in 0..u8::try_from(Section::DEPTH).unwrap() {
            for x in 0..u8::try_from(Section::WIDTH).unwrap() {
                // Check that the iterator position matches the expected position.
                assert_eq!(iter.position(), ChunkBlockPosition::new(x, y, z));

                // Check that the block is air.
                assert_eq!(iter.next(), Some(0), "Block is not air");
            }
        }
    }

    // Check that the iterator is finished.
    assert!(iter.is_finished(), "Iterator is not finished");
    assert_eq!(iter.next(), None, "Iterator returned a value after finishing");
    assert_eq!(
        iter.position(),
        ChunkBlockPosition::new(15, chunk.height - 1, 15),
        "Iterator position is not at the end"
    );
}
