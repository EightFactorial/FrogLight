//! Quick and dirty benchmarks for using `Chunk::contains_raw_block` vs
//! iterating over all blocks.
#![allow(clippy::large_stack_arrays, reason = "Ignored")]

#[cfg(any(feature = "froglight-block", feature = "froglight-biome"))]
use core::any::TypeId;

use bitvec::vec::BitVec;
use divan::prelude::*;
#[cfg(feature = "froglight-biome")]
use froglight_biome::{
    biome::{BiomeAttributeSet, BiomeMetadata},
    prelude::*,
    storage::BiomeStorage,
};
#[cfg(feature = "froglight-block")]
use froglight_block::{block::BlockMetadata, prelude::*, storage::BlockStorage};
#[cfg(any(feature = "froglight-biome", feature = "froglight-block"))]
use froglight_common::prelude::*;
use froglight_world::{
    naive::{
        NaiveChunk,
        storage::{ArrayChunkStorage, ChunkStorage},
    },
    section::{Section, SectionData, SectionPalette},
};
use smallvec::SmallVec;

fn main() { divan::main() }

macro_rules! create {
    (@blocks $($tt:tt)*) => {{
        black_box(NaiveChunk::new(ChunkStorage::Large(ArrayChunkStorage::new(
            core::array::from_fn(|_| unsafe {
                Section::new_unchecked(
                    0,
                    0,
                    $($tt)*,
                    SectionData::new_unchecked(
                        0,
                        SectionPalette::Single(0),
                        BitVec::EMPTY,
                    ),
                )
            }),
        ))))
    }};
    (@biomes $($tt:tt)*) => {{
        black_box(NaiveChunk::new(ChunkStorage::Large(ArrayChunkStorage::new(
            core::array::from_fn(|_| unsafe {
                Section::new_unchecked(
                    0,
                    0,
                    SectionData::new_unchecked(
                        0,
                        SectionPalette::Single(0),
                        BitVec::EMPTY,
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
        SectionData::new_unchecked(
            0,
            SectionPalette::Single(0),
            BitVec::EMPTY,
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
        SectionData::new_unchecked(
            0,
            SectionPalette::Single(0),
            BitVec::EMPTY,
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
        SectionData::new_unchecked(
            0,
            SectionPalette::Single(0),
            BitVec::EMPTY,
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
        SectionData::new_unchecked(
            0,
            SectionPalette::Single(0),
            BitVec::EMPTY,
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
        SectionData::new_unchecked(
            1,
            SectionPalette::Vector(SmallVec::from_vec(vec![0, 1])),
            BitVec::from_slice(&[0; 4096]),
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
        SectionData::new_unchecked(
            1,
            SectionPalette::Vector(SmallVec::from_vec(vec![0, 1])),
            BitVec::from_slice(&[0; 4096]),
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
        SectionData::new_unchecked(
            1,
            SectionPalette::Vector(SmallVec::from_vec(vec![0])),
            BitVec::from_slice(&[0; 4096]),
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
        SectionData::new_unchecked(
            1,
            SectionPalette::Vector(SmallVec::from_vec(vec![0, 1])),
            BitVec::from_slice(&[0; 4096]),
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
        SectionData::new_unchecked(
            1,
            SectionPalette::Vector(SmallVec::from_vec(vec![0, 1])),
            BitVec::from_slice(&[0; 4096]),
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
        SectionData::new_unchecked(
            1,
            SectionPalette::Global,
            BitVec::from_slice(&[0; 4096]),
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
        SectionData::new_unchecked(
            1,
            SectionPalette::Global,
            BitVec::from_slice(&[0; 4096]),
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
        SectionData::new_unchecked(
            1,
            SectionPalette::Global,
            BitVec::from_slice(&[0; 4096]),
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
        SectionData::new_unchecked(
            1,
            SectionPalette::Global,
            BitVec::from_slice(&[0; 4096]),
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
                GlobalBiomeId::new(0),
                0,
                0,
                0,
                0,
                true,
                0.0,
                0.0,
                &BIOME_DATA,
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
                GlobalBiomeId::new(1),
                0,
                0,
                0,
                0,
                true,
                0.0,
                0.0,
                &BIOME_DATA,
            )
        };

        &STATIC
    };
}

#[cfg(feature = "froglight-biome")]
static BIOME_DATA: froglight_biome::version::LazyLock<BiomeAttributeSet> =
    froglight_biome::version::LazyLock::new(BiomeAttributeSet::empty);

#[cfg(feature = "froglight-biome")]
froglight_biome::version::version_implement! {
    impl BiomeVersion => TestVersion {
        const BIOMES: BiomeStorage;
        fn new_biomes() => {
            unsafe {
                BiomeStorage::build::<Self>(&[
                    Plains::METADATA,
                    Forest::METADATA,
                ])
            }
        }
    }
}

#[divan::bench]
#[cfg(feature = "froglight-biome")]
fn contains_global_biome_best(b: Bencher) {
    // An empty section with no biomes.
    let global = create! {
        @biomes
        SectionData::new_unchecked(
            1,
            SectionPalette::Global,
            BitVec::from_slice(&[0; 4096]),
        )
    };

    let biomes = TestVersion::biomes();
    b.bench(|| {
        black_box(global.contains_biome_type(TypeId::of::<Plains>(), biomes));
    });
}

#[divan::bench]
#[cfg(feature = "froglight-biome")]
fn contains_global_biome_worst(b: Bencher) {
    // An empty section with no biomes.
    let global = create! {
        @biomes
        SectionData::new_unchecked(
            1,
            SectionPalette::Global,
            BitVec::from_slice(&[0; 4096]),
        )
    };

    let biomes = TestVersion::biomes();
    b.bench(|| {
        black_box(global.contains_biome_type(TypeId::of::<Forest>(), biomes));
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
                Identifier::new_static("test:air"),
                GlobalStateId::new(0),
                RelativeStateId::new(0),
            )
        };
        &STATIC
    };

    fn is_air(_: RelativeStateId) -> bool { true }

    fn is_solid(_: RelativeStateId) -> bool { false }

    fn is_transparent(_: RelativeStateId) -> bool { true }
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
            use froglight_common::prelude::Identifier;

            BlockMetadata::new::<Stone, TestVersion>(
                Identifier::new_static("test:stone"),
                GlobalStateId::new(1),
                RelativeStateId::new(0),
            )
        };
        &STATIC
    };
}

#[cfg(feature = "froglight-block")]
froglight_block::version::version_implement! {
    impl BlockVersion => TestVersion {
        const BLOCKS: BlockStorage;
        fn new_blocks() => {
            unsafe {
                BlockStorage::build::<Self>(Box::new([
                    Air::METADATA,
                    Stone::METADATA,
                ]))
            }
        }
    }
}

#[divan::bench]
#[cfg(feature = "froglight-block")]
fn contains_global_block_best(b: Bencher) {
    // An empty section with no blocks.
    let global = create! {
        @blocks
        SectionData::new_unchecked(
            1,
            SectionPalette::Global,
            BitVec::from_slice(&[0; 4096]),
        )
    };

    let blocks = TestVersion::blocks();
    b.bench(|| {
        black_box(global.contains_block_type(TypeId::of::<Air>(), blocks));
    });
}

#[divan::bench]
#[cfg(feature = "froglight-block")]
fn contains_global_block_worst(b: Bencher) {
    // An empty section with no blocks.
    let global = create! {
        @blocks
        SectionData::new_unchecked(
            1,
            SectionPalette::Global,
            BitVec::from_slice(&[0; 4096]),
        )
    };

    let blocks = TestVersion::blocks();
    b.bench(|| {
        black_box(global.contains_block_type(TypeId::of::<Stone>(), blocks));
    });
}
