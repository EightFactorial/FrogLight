use std::marker::PhantomData;

use froglight_core::common::ChunkBlockPosition;
use froglight_protocol::traits::Version;

use super::Chunk;
use crate::world::Section;

/// An iterator over all of the blocks in a [`Chunk`].
///
/// Returns block ids.
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
        // If the position is out of bounds, always return None.
        if self.finished {
            return None;
        }

        // Get the block id at the current position.
        let result = self.chunk.get_blockid(&self.position);

        if usize::from(self.position.x) == Section::WIDTH - 1
            && usize::from(self.position.z) == Section::DEPTH - 1
            && self.position.y == self.chunk.height - 1
        {
            // Mark the iterator as finished.
            self.finished = true;
        } else if result.is_some() {
            // Move to the next position if the current block exists.
            self.position.x += 1;
            if usize::from(self.position.x) >= Section::WIDTH {
                self.position.x = 0;
                self.position.z += 1;
                if usize::from(self.position.z) >= Section::DEPTH {
                    self.position.z = 0;
                    self.position.y += 1;
                }
            }
        }

        result
    }
}

/// An iterator over all of the blocks in a [`Chunk`].
///
/// Returns blocks.
#[derive(Debug)]
pub struct ChunkBlockIterator<'c, V: Version> {
    iterator: ChunkIdIterator<'c>,
    phantom: PhantomData<V>,
}

impl<'c, V: Version> ChunkBlockIterator<'c, V> {
    /// Creates a new [`ChunkBlockIterator`] for the given [`Chunk`].
    #[must_use]
    pub const fn new(chunk: &'c Chunk) -> Self {
        Self { iterator: ChunkIdIterator::new(chunk), phantom: PhantomData }
    }

    /// Creates a new [`ChunkBlockIterator`] for the given [`Chunk`] starting at
    /// the given position.
    #[must_use]
    pub const fn starting_at(chunk: &'c Chunk, position: ChunkBlockPosition) -> Self {
        Self { iterator: ChunkIdIterator::starting_at(chunk, position), phantom: PhantomData }
    }

    /// Gets the current position of the iterator.
    #[must_use]
    pub const fn position(&self) -> ChunkBlockPosition { self.iterator.position() }

    /// Returns whether the iterator is finished.
    #[must_use]
    pub const fn is_finished(&self) -> bool { self.iterator.is_finished() }
}

impl<V: Version> Iterator for ChunkBlockIterator<'_, V> {
    // TODO: Return blocks instead of block ids.
    type Item = usize;
    #[inline]
    #[allow(clippy::bind_instead_of_map)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().and_then(|_id| todo!("Block::from_id(id)"))
    }
}

#[test]
fn chunkid_iter() {
    // Create a new empty chunk and get an iterator over its blocks.
    let chunk = Chunk::new_empty(320, -64);
    let mut iter = chunk.blockid_iter();

    // Check that all values are 0.
    for y in 0..chunk.height {
        for z in 0..Section::DEPTH {
            for x in 0..Section::WIDTH {
                // Check that the iterator position matches the expected position.
                assert_eq!(
                    iter.position(),
                    ChunkBlockPosition::new(x.try_into().unwrap(), y, z.try_into().unwrap()),
                    "Iterator position does not match expected position"
                );

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

// #[test]
// fn chunkblock_iter() {
//     use froglight_protocol::versions::v1_20_0::V1_20_0;

//     // Create a new empty chunk and get an iterator over its blocks.
//     let chunk = Chunk::new_empty(320, -64);
//     let mut iter = chunk.block_iter::<V1_20_0>();

//     // Check that all values are 0.
//     for y in 0..chunk.height {
//         for z in 0..Section::DEPTH {
//             for x in 0..Section::WIDTH {
//                 // Check that the iterator position matches the expected
// position.                 assert_eq!(
//                     iter.position(),
//                     ChunkBlockPosition::new(x.try_into().unwrap(), y,
// z.try_into().unwrap()),                     "Iterator position does not match
// expected position"                 );

//                 // Check that the block is air.
//                 assert_eq!(iter.next(), Some(0), "Block is not air");
//             }
//         }
//     }

//     // Check that the iterator is finished.
//     assert!(iter.is_finished(), "Iterator is not finished");
//     assert_eq!(iter.next(), None, "Iterator returned a value after
// finishing");     assert_eq!(
//         iter.position(),
//         ChunkBlockPosition::new(15, chunk.height - 1, 15),
//         "Iterator position is not at the end"
//     );
// }
