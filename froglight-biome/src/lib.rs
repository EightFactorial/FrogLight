#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub(crate) mod atomic;
pub mod biome;
pub mod generated;
pub mod storage;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        biome::{AttributeType, Biome, BiomeType, FeatureType},
        generated::{attribute as biome_attribute, biome, feature as biome_feature},
        version::BiomeVersion,
    };
}
