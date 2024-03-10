use froglight_core::common::SectionBlockPosition;

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
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // If the iterator is finished, return `None`.
        if self.finished {
            return None;
        }

        // Get the block id at the current position.
        let result = self.section.get_blockid(self.position);

        if self.position == SectionBlockPosition::MAX {
            // Mark the iterator as finished.
            self.finished = true;
        } else {
            self.position = self.position.next_wrapping();
        }

        Some(result)
    }
}

#[test]
fn section_iter() {
    let section = Section::default();
    let mut iter = section.block_iter();

    // Check that all values are 0.
    for y in 0..u8::try_from(Section::HEIGHT).unwrap() {
        for z in 0..u8::try_from(Section::DEPTH).unwrap() {
            for x in 0..u8::try_from(Section::WIDTH).unwrap() {
                // Check that the iterator position matches the expected position.
                assert_eq!(iter.position(), SectionBlockPosition::new(x, y, z));

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
