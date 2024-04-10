use froglight_protocol::common::SectionBlockPosition;

use crate::ChunkSection;

/// An iterator over the blocks of a [`ChunkSection`].
#[derive(Debug, Clone)]
pub struct SectionBlockIter<'s> {
    section: &'s ChunkSection,
    index: SectionBlockPosition,
    finished: bool,
}

impl<'s> SectionBlockIter<'s> {
    /// Creates a new [`ChunkSection`] block iterator.
    #[must_use]
    pub fn new(section: &'s ChunkSection) -> Self {
        Self { section, index: SectionBlockPosition::default(), finished: false }
    }

    /// Returns the current index of the iterator.
    #[must_use]
    pub fn index(&self) -> SectionBlockPosition { self.index }

    /// Returns whether the iterator has finished.
    #[must_use]
    pub fn finished(&self) -> bool { self.finished }

    /// Resets the iterator to the
    /// [`default position`](SectionBlockPosition::default).
    pub fn reset(&mut self) {
        self.index = SectionBlockPosition::default();
        self.finished = false;
    }
}

impl Iterator for SectionBlockIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let block = self.section.get_block(self.index);

        if let Some(next) = self.index.next() {
            self.index = next;
        } else {
            self.finished = true;
        }

        Some(block)
    }
}

#[test]
fn section_iter() {
    let section = ChunkSection::default();
    let mut iter = section.block_iter();

    // Check that all values are 0.
    for y in 0..u8::try_from(ChunkSection::HEIGHT).unwrap() {
        for z in 0..u8::try_from(ChunkSection::DEPTH).unwrap() {
            for x in 0..u8::try_from(ChunkSection::WIDTH).unwrap() {
                // Check that the iterator position matches the expected position.
                assert_eq!(iter.index(), SectionBlockPosition::new(x, y, z));

                // Check that the block is air.
                assert_eq!(iter.next(), Some(0), "Block is not air");
            }
        }
    }

    // Check that the iterator is finished.
    assert!(iter.finished(), "Iterator is not finished");
    assert_eq!(iter.next(), None, "Iterator returned a value after finishing");
    assert_eq!(
        iter.index(),
        SectionBlockPosition::MAX,
        "Iterator position is not at the maximum position"
    );
}
