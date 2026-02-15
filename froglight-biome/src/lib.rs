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

    #[cfg(feature = "biome_data")]
    pub use crate::biome::AttributeType;
    #[cfg(feature = "biome_data")]
    pub use crate::generated::attribute as biome_attribute;
    pub use crate::{
        biome::{Biome, BiomeType},
        generated::biome::{self, VanillaBiome},
        version::BiomeVersion,
    };
}
