//! Quick and dirty benchmarks for using `Chunk::contains_raw_block` vs
//! iterating over all blocks.
#![allow(clippy::large_stack_arrays, reason = "Ignored")]

#[cfg(feature = "froglight-block")]
use core::any::TypeId;

use bitvec::slice::BitSlice;
use divan::prelude::*;
#[cfg(feature = "froglight-biome")]
use froglight_biome::{biome::BiomeMetadata, implement_biomes, prelude::*, storage::BiomeStorage};
#[cfg(feature = "froglight-block")]
use froglight_block::{
    block::{BlockBehavior, BlockMetadata, StateId},
    prelude::*,
    storage::BlockStorage,
};
#[cfg(any(feature = "froglight-biome", feature = "froglight-block"))]
use froglight_common::prelude::*;
use froglight_world::borrowed::{
    BorrowedChunk, BorrowedSection,
    section::{BorrowedPalette, BorrowedSectionData},
    storage::{BorrowedArrayStorage, BorrowedChunkStorage},
};

fn main() { divan::main() }

macro_rules! create {
    (@blocks $($tt:tt)*) => {{
        black_box(BorrowedChunk::new(BorrowedChunkStorage::Large(BorrowedArrayStorage::new(
            core::array::from_fn(|_| unsafe {
                BorrowedSection::new_unchecked(
                    0,
                    $($tt)*,
                    BorrowedSectionData::new_unchecked(
                        0,
                        BorrowedPalette::Single(0),
                        BitSlice::empty(),
                    ),
                )
            }),
        ))))
    }};
    (@biomes $($tt:tt)*) => {{
        black_box(BorrowedChunk::new(BorrowedChunkStorage::Large(BorrowedArrayStorage::new(
            core::array::from_fn(|_| unsafe {
                BorrowedSection::new_unchecked(
                    0,
                    BorrowedSectionData::new_unchecked(
                        0,
                        BorrowedPalette::Single(0),
                        BitSlice::empty(),
                    ),
                    $($tt)*,
                )
            }),
        ))))
    }};
}

#[divan::bench]
fn contains_single_best(b: Bencher) {
    // An empty section with no blocks.
    let single = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            0,
            BorrowedPalette::Single(0),
            BitSlice::empty(),
        )
    };

    b.bench(|| {
        black_box(single.contains_raw_block(0));
    });
}

#[divan::bench]
fn contains_single_worst(b: Bencher) {
    // An empty section with no blocks.
    let single = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            0,
            BorrowedPalette::Single(0),
            BitSlice::empty(),
        )
    };

    b.bench(|| {
        black_box(single.contains_raw_block(1));
    });
}

#[divan::bench]
fn contains_single_best_iter(b: Bencher) {
    // An empty section with no blocks.
    let single = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            0,
            BorrowedPalette::Single(0),
            BitSlice::empty(),
        )
    };

    b.bench(|| {
        black_box(single.iter_raw_blocks().any(|id| id == 0));
    });
}

#[divan::bench]
fn contains_single_worst_iter(b: Bencher) {
    // An empty section with no blocks.
    let single = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            0,
            BorrowedPalette::Single(0),
            BitSlice::empty(),
        )
    };

    b.bench(|| {
        black_box(single.iter_raw_blocks().any(|id| id == 1));
    });
}

#[divan::bench]
fn contains_vector_best(b: Bencher) {
    // An empty section with no blocks.
    let vector = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Vector(&[0, 1]),
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(vector.contains_raw_block(0));
    });
}

#[divan::bench]
fn contains_vector_best_iter(b: Bencher) {
    // An empty section with no blocks.
    let vector = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Vector(&[0, 1]),
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(vector.iter_raw_blocks().any(|id| id == 0));
    });
}

#[divan::bench]
fn contains_vector_pass(b: Bencher) {
    // An empty section with no blocks.
    let vector = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Vector(&[0]),
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(vector.contains_raw_block(1));
    });
}

#[divan::bench]
fn contains_vector_worst_iter(b: Bencher) {
    // An empty section with no blocks.
    let vector = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Vector(&[0, 1]),
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(vector.iter_raw_blocks().any(|id| id == 2));
    });
}

#[divan::bench]
fn contains_vector_worst(b: Bencher) {
    // An empty section with no blocks.
    let vector = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Vector(&[0, 1]),
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(vector.contains_raw_block(1));
    });
}

#[divan::bench]
fn contains_global_best(b: Bencher) {
    // An empty section with no blocks.
    let global = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Global,
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(global.contains_raw_block(0));
    });
}

#[divan::bench]
fn contains_global_worst(b: Bencher) {
    // An empty section with no blocks.
    let global = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Global,
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(global.contains_raw_block(1));
    });
}

#[divan::bench]
fn contains_global_best_iter(b: Bencher) {
    // An empty section with no blocks.
    let global = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Global,
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(global.iter_raw_blocks().any(|id| id == 0));
    });
}

#[divan::bench]
fn contains_global_worst_iter(b: Bencher) {
    // An empty section with no blocks.
    let global = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Global,
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(global.iter_raw_blocks().any(|id| id == 1));
    });
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg(any(feature = "froglight-biome", feature = "froglight-block"))]
struct TestVersion;

#[cfg(any(feature = "froglight-biome", feature = "froglight-block"))]
impl Version for TestVersion {
    const DATA_VERSION: u32 = u32::MIN;
    const PROTOCOL_ID: u32 = u32::MIN;
    const RESOURCE_VERSION: u32 = u32::MIN;
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg(feature = "froglight-biome")]
struct Plains;

#[cfg(feature = "froglight-biome")]
impl BiomeType<TestVersion> for Plains {
    const METADATA: &'static BiomeMetadata = {
        static STATIC: BiomeMetadata = unsafe {
            BiomeMetadata::new::<Plains, TestVersion>(
                Identifier::new_static("test:plains"),
                0,
                0,
                0,
                0,
                0,
                true,
                0.0,
                0.0,
            )
        };
        &STATIC
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg(feature = "froglight-biome")]
struct Forest;

#[cfg(feature = "froglight-biome")]
impl BiomeType<TestVersion> for Forest {
    const METADATA: &'static BiomeMetadata = {
        static STATIC: BiomeMetadata = unsafe {
            BiomeMetadata::new::<Forest, TestVersion>(
                Identifier::new_static("test:forest"),
                1,
                0,
                0,
                0,
                0,
                true,
                0.0,
                0.0,
            )
        };
        &STATIC
    };
}

#[cfg(feature = "froglight-biome")]
implement_biomes! {
    TestVersion => unsafe {
        BiomeStorage::new_static(&[
            Plains::METADATA,
            Forest::METADATA,
        ])
    }
}

#[divan::bench]
#[cfg(feature = "froglight-biome")]
fn contains_global_biome_best(b: Bencher) {
    // An empty section with no biomes.
    let global = create! {
        @biomes
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Global,
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(global.contains_biome_type::<Plains, TestVersion>());
    });
}

#[divan::bench]
#[cfg(feature = "froglight-biome")]
fn contains_global_biome_worst(b: Bencher) {
    // An empty section with no biomes.
    let global = create! {
        @biomes
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Global,
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(global.contains_biome_type::<Forest, TestVersion>());
    });
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg(feature = "froglight-block")]
struct Air;

#[cfg(feature = "froglight-block")]
impl BlockType<TestVersion> for Air {
    type Attributes = ();

    const ATTRDATA: &'static [(&'static str, TypeId)] = &[];
    const METADATA: &'static BlockMetadata = {
        static STATIC: BlockMetadata = unsafe {
            BlockMetadata::new::<Air, TestVersion>(
                Identifier::new_unchecked("test:air"),
                0,
                0,
                BlockBehavior::new::<Air, TestVersion>(),
            )
        };
        &STATIC
    };

    fn is_air(_: StateId) -> bool { true }

    fn is_solid(_: StateId) -> bool { false }

    fn is_transparent(_: StateId) -> bool { true }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg(feature = "froglight-block")]
struct Stone;

#[cfg(feature = "froglight-block")]
impl BlockType<TestVersion> for Stone {
    type Attributes = ();

    const ATTRDATA: &'static [(&'static str, TypeId)] = &[];
    const METADATA: &'static BlockMetadata = {
        static STATIC: BlockMetadata = unsafe {
            use froglight_block::block::BlockBehavior;
            use froglight_common::prelude::Identifier;

            BlockMetadata::new::<Stone, TestVersion>(
                Identifier::new_unchecked("test:stone"),
                1,
                0,
                BlockBehavior::new::<Stone, TestVersion>(),
            )
        };
        &STATIC
    };
}

#[cfg(feature = "froglight-block")]
froglight_block::implement_blocks! {
    TestVersion => unsafe {
        BlockStorage::new_static(&[
            Air::METADATA,
            Stone::METADATA,
        ])
    }
}

#[divan::bench]
#[cfg(feature = "froglight-block")]
fn contains_global_block_best(b: Bencher) {
    // An empty section with no blocks.
    let global = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Global,
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(global.contains_block_type::<Air, TestVersion>());
    });
}

#[divan::bench]
#[cfg(feature = "froglight-block")]
fn contains_global_block_worst(b: Bencher) {
    // An empty section with no blocks.
    let global = create! {
        @blocks
        BorrowedSectionData::new_unchecked(
            1,
            BorrowedPalette::Global,
            BitSlice::from_slice(&[0; 4096]),
        )
    };

    b.bench(|| {
        black_box(global.contains_block_type::<Stone, TestVersion>());
    });
}
