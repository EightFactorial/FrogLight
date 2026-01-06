//! TODO

use bitvec::slice::BitSlice;
use froglight_world::{
    SECTION_VOLUME,
    borrowed::{
        BorrowedSection,
        section::{BorrowedPalette, BorrowedSectionData},
    },
    component::SectionBlockPos,
};

#[test]
fn empty() {
    // An empty section with no blocks.
    let section = unsafe {
        BorrowedSection::new_unchecked(
            0,
            BorrowedSectionData::new_unchecked(0, BorrowedPalette::Single(0), BitSlice::empty()),
            BorrowedSectionData::new_unchecked(0, BorrowedPalette::Single(0), BitSlice::empty()),
        )
    };

    assert_eq!(section.block_count(), 0);
    assert_eq!(section.block_data().bits_per_entry(), 0);
    assert_eq!(section.biome_data().bits_per_entry(), 0);

    for index in 0..SECTION_VOLUME {
        let pos = SectionBlockPos::new_index(index);
        assert_eq!(section.get_raw_block(pos), 0);
        assert_eq!(section.block_data().get_index(usize::from(index)), Some(0));
    }

    for index in 0..SECTION_VOLUME / (4 * 4 * 4) {
        let pos = SectionBlockPos::new_index(index);
        assert_eq!(section.get_raw_biome(pos), 0);
        assert_eq!(section.biome_data().get_index(usize::from(index)), Some(0));
    }
}
