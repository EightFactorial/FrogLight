//! Additional methods that require the [`froglight_biome`] crate.
#![allow(unused_imports, reason = "WIP")]

use core::any::TypeId;

#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
use froglight_biome::biome::BiomeType;
use froglight_biome::{biome::GlobalId, prelude::*, storage::BiomeStorage};

use crate::{
    borrowed::{BorrowedChunk, section::BorrowedPalette},
    component::ChunkBlockPos,
    prelude::*,
};

impl BorrowedChunk<'_> {}
