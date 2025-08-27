//! TODO

use bitvec::vec::BitVec;
use froglight_world::{
    palette::SectionPalette,
    prelude::*,
    section::SectionData,
    storage::{ArrayStorage, SectionStorage, VecStorage},
};

/// Ensure that the storage size calculations are correct.
#[test]
fn storage_size() {
    assert_eq!(SectionStorage::empty_normal().min_height(), 0);
    assert_eq!(SectionStorage::empty_large().min_height(), -64);
    assert_eq!(ArrayStorage::<16, 0>::empty().min_height(), 0);
    assert_eq!(ArrayStorage::<24, -64>::empty().min_height(), -64);
    // Contains no sections, so the min height is the same as the offset.
    assert_eq!(SectionStorage::new(Vec::new(), 0).min_height(), 0);
    assert_eq!(VecStorage::new(Vec::new(), 0).min_height(), 0);
    assert_eq!(VecStorage::new(Vec::new(), -64).min_height(), -64);

    assert_eq!(SectionStorage::empty_normal().max_height(), 256);
    assert_eq!(SectionStorage::empty_large().max_height(), 320);
    assert_eq!(ArrayStorage::<16, 0>::empty().max_height(), 256);
    assert_eq!(ArrayStorage::<24, -64>::empty().max_height(), 320);
    // Contains no sections, so the max height is the same as the offset.
    assert_eq!(SectionStorage::new(Vec::new(), 0).max_height(), 0);
    assert_eq!(VecStorage::new(Vec::new(), 0).max_height(), 0);
    assert_eq!(VecStorage::new(Vec::new(), -64).max_height(), -64);

    assert_eq!(SectionStorage::empty_normal().volume(), 16 * Section::VOLUME);
    assert_eq!(SectionStorage::empty_large().volume(), 24 * Section::VOLUME);
    assert_eq!(ArrayStorage::<16, 0>::empty().volume(), 16 * Section::VOLUME);
    assert_eq!(ArrayStorage::<24, -64>::empty().volume(), 24 * Section::VOLUME);
    // Contains no sections, so the volume is 0.
    assert_eq!(SectionStorage::new(Vec::new(), 0).volume(), 0);
    assert_eq!(VecStorage::new(Vec::new(), 0).volume(), 0);
    assert_eq!(VecStorage::new(Vec::new(), -64).volume(), 0);
}

/// Ensure that empty sections can be indexed into correctly without panicking.
#[test]
fn section_air_index() {
    fn assert_air(section: &Section) {
        let (blocks, biomes) = Section::data(section);
        for index in 0..=Section::VOLUME {
            assert_eq!(blocks.get(index), 0);
            assert_eq!(biomes.get(index), 0);
        }
    }

    // Test: SectionPalette::Single
    assert_air(&Section::AIR);

    // Test: SectionPalette::Vector
    assert_air(&unsafe {
        Section::from_parts_unchecked(
            0,
            SectionData::from_parts_unchecked(
                1,
                BitVec::from_vec(vec![0u64; 4096 / 64]),
                SectionPalette::Vector(vec![0]),
            ),
            SectionData::from_parts_unchecked(
                1,
                BitVec::from_vec(vec![0u64; 1024 / 64]),
                SectionPalette::Vector(vec![0]),
            ),
        )
    });

    // Test: SectionPalette::Global
    assert_air(&unsafe {
        Section::from_parts_unchecked(
            0,
            SectionData::from_parts_unchecked(
                12,
                BitVec::from_vec(vec![0u64; 4096 * 12 / 64]),
                SectionPalette::Global,
            ),
            SectionData::from_parts_unchecked(
                6,
                BitVec::from_vec(vec![0u64; 1024 * 6 / 64]),
                SectionPalette::Global,
            ),
        )
    });
}
