use crate::{position::SectionBlockPos, prelude::*};

/// Test a `SectionPalette::Single`.
#[test]
fn section_single() {
    let section = Section::default();
    assert_eq!(section.blocks(), 0);

    // Check all blocks
    for index in 0..4096 {
        assert_eq!(section.get_block(SectionBlockPos::from_index(index)), 0);
    }
}

/// Test growing a `SectionPalette::Single`
/// into a `SectionPalette::Vector`.
#[test]
fn section_vector() {
    let mut section = Section::default();
    assert_eq!(section.blocks(), 0);

    // Set all blocks and check them
    for index in 0..256 {
        let pos = SectionBlockPos::from_index(index);
        assert_eq!(section.set_block(pos, index as u32), 0, "Weird return value at {index}?");
        assert_eq!(section.get_block(pos), index as u32, "Invalid block at {index}!");
    }
    // Check all blocks one more time
    for index in 0..256 {
        let pos = SectionBlockPos::from_index(index);
        assert_eq!(section.get_block(pos), index as u32, "Invalid block at {index}!");
    }
}

/// Test growing a `SectionPalette::Single`
/// all the way into a `SectionPalette::Global`.
#[test]
fn section_global() {
    let mut section = Section::default();
    assert_eq!(section.blocks(), 0);

    // Set all blocks and check them
    for index in 0..4096 {
        let pos = SectionBlockPos::from_index(index);
        assert_eq!(section.set_block(pos, index as u32), 0, "Weird return value at {index}?");
        assert_eq!(section.get_block(pos), index as u32, "Invalid block at {index}!");
    }
    // Check all blocks one more time
    for index in 0..4096 {
        let pos = SectionBlockPos::from_index(index);
        assert_eq!(section.get_block(pos), index as u32, "Invalid block at {index}!");
    }
}
