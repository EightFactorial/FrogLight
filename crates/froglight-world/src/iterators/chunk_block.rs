use froglight_protocol::common::ChunkBlockPosition;

use crate::Chunk;

/// An iterator over the blocks of a [`Chunk`].
#[derive(Debug, Clone)]
pub struct ChunkBlockIter<'c> {
    chunk: &'c Chunk,
    index: ChunkBlockPosition,
    finished: bool,
}

impl<'c> ChunkBlockIter<'c> {
    /// Creates a new [`Chunk`] block iterator.
    #[must_use]
    pub fn new(chunk: &'c Chunk) -> Self {
        Self { chunk, index: ChunkBlockPosition::default(), finished: false }
    }

    /// Returns the current index of the iterator.
    #[must_use]
    pub fn index(&self) -> ChunkBlockPosition { self.index }

    /// Resets the iterator to the
    /// [`default position`](ChunkBlockPosition::default).
    pub fn reset(&mut self) {
        self.index = ChunkBlockPosition::default();
        self.finished = false;
    }
}

impl Iterator for ChunkBlockIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let block = self.chunk.get_block(self.index);

        let next_index = self.index.next();
        if next_index.y() >= self.chunk.height() {
            self.finished = true;
        } else {
            self.index = next_index;
        }

        block
    }
}

#[test]
fn chunkid_iter() {
    use crate::section::ChunkSection;

    // Create a new empty chunk and get an iterator over its blocks.
    let chunk = Chunk::new_empty(320, -64);
    let mut iter = chunk.block_iter();

    // Check that all values are 0.
    for y in 0..chunk.height() {
        for z in 0..u8::try_from(ChunkSection::DEPTH).unwrap() {
            for x in 0..u8::try_from(ChunkSection::WIDTH).unwrap() {
                // Check that the iterator position matches the expected position.
                assert_eq!(iter.index(), ChunkBlockPosition::new(x, y, z));

                // Check that the block is air.
                assert_eq!(iter.next(), Some(0), "Block is not air");
            }
        }
    }

    // Check that the iterator is finished.
    assert_eq!(iter.next(), None, "Iterator returned a value after finishing");
    assert_eq!(
        iter.index(),
        ChunkBlockPosition::new(15, chunk.height() - 1, 15),
        "Iterator position is not at the end"
    );
}
