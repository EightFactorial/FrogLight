use std::marker::PhantomData;

use froglight_core::common::SectionBlockPosition;
use froglight_protocol::traits::Version;

use super::Section;

/// An iterator over all of the blocks in a [`Section`].
///
/// Returns block ids.
#[derive(Debug)]
pub struct SectionIdIterator<'s> {
    section: &'s Section,
    position: SectionBlockPosition,
    finished: bool,
}

impl<'s> SectionIdIterator<'s> {
    /// Creates a new [`SectionIdIterator`] for the given [`Section`].
    #[must_use]
    pub const fn new(section: &'s Section) -> Self {
        Self { section, position: SectionBlockPosition::ZERO, finished: false }
    }

    /// Creates a new [`SectionIdIterator`] for the given [`Section`]
    /// starting at the given position.
    #[must_use]
    pub const fn starting_at(section: &'s Section, position: SectionBlockPosition) -> Self {
        Self { section, position, finished: false }
    }

    /// Gets the current position of the iterator.
    #[must_use]
    pub const fn position(&self) -> SectionBlockPosition { self.position }

    /// Returns whether the iterator is finished.
    #[must_use]
    pub const fn is_finished(&self) -> bool { self.finished }
}

impl Iterator for SectionIdIterator<'_> {
    // TODO: Return blocks instead of block ids.
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // If the iterator is finished, always return None.
        if self.finished {
            return None;
        }

        // Get the block id at the current position.
        let result = self.section.get_blockid(self.position);

        if self.position == SectionBlockPosition::MAX {
            // Mark the iterator as finished.
            self.finished = true;
        } else {
            // Move to the next position.
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

        Some(result)
    }
}

/// An iterator over all of the blocks in a [`Section`].
///
/// Returns blocks.
#[derive(Debug)]
pub struct SectionBlockIterator<'s, V: Version> {
    iterator: SectionIdIterator<'s>,
    phantom: PhantomData<V>,
}

impl<'s, V: Version> SectionBlockIterator<'s, V> {
    /// Creates a new [`SectionBlockIterator`] for the given [`Section`].
    #[must_use]
    pub const fn new(section: &'s Section) -> Self {
        Self { iterator: SectionIdIterator::new(section), phantom: PhantomData }
    }

    /// Creates a new [`SectionBlockIterator`] for the given [`Section`]
    /// starting at the given position.
    #[must_use]
    pub const fn starting_at(section: &'s Section, position: SectionBlockPosition) -> Self {
        Self { iterator: SectionIdIterator::starting_at(section, position), phantom: PhantomData }
    }

    /// Gets the current position of the iterator.
    #[must_use]
    pub fn position(&self) -> SectionBlockPosition { self.iterator.position() }

    /// Returns whether the iterator is finished.
    #[must_use]
    pub fn is_finished(&self) -> bool { self.iterator.is_finished() }
}

impl<V: Version> Iterator for SectionBlockIterator<'_, V> {
    // TODO: Return blocks instead of block ids.
    type Item = usize;
    #[inline]
    #[allow(clippy::bind_instead_of_map)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().and_then(|_id| todo!("Block::from_id(id)"))
    }
}

#[test]
fn sectionid_iter() {
    let section = Section::default();
    let mut iter = section.blockid_iter();

    // Check that all values are 0.
    for y in 0..Section::HEIGHT {
        for z in 0..Section::DEPTH {
            for x in 0..Section::WIDTH {
                // Check that the iterator position matches the expected position.
                assert_eq!(
                    iter.position(),
                    SectionBlockPosition::new(
                        u8::try_from(x).unwrap(),
                        u8::try_from(y).unwrap(),
                        u8::try_from(z).unwrap()
                    ),
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
        SectionBlockPosition::MAX,
        "Iterator position is not at the maximum position"
    );
}

// #[test]
// fn sectionblock_iter() {
//     use froglight_protocol::versions::v1_20_0::V1_20_0;

//     let section = Section::default();
//     let mut iter = section.block_iter::<V1_20_0>();

//     // Check that all values are 0.
//     for y in 0..Section::HEIGHT {
//         for z in 0..Section::DEPTH {
//             for x in 0..Section::WIDTH {
//                 // Check that the iterator position matches the expected
// position.                 assert_eq!(
//                     iter.position(),
//                     SectionBlockPosition::new(
//                         u8::try_from(x).unwrap(),
//                         u8::try_from(y).unwrap(),
//                         u8::try_from(z).unwrap()
//                     ),
//                     "Iterator position does not match expected position"
//                 );

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
//         SectionBlockPosition::MAX,
//         "Iterator position is not at the maximum position"
//     );
// }
