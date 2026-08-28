#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "facet")]
pub mod facet;
pub mod types;
pub mod unicode;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "froglight-facet")]
    pub use crate::facet::froglight_facet::SnbtTemplate;
    pub use crate::types::indexed::{
        IndexedSnbt,
        core::{IndexedSnbtCow, IndexedSnbtSlice},
    };
    #[cfg(feature = "facet")]
    pub use crate::{
        self as snbt,
        facet::{
            deserialize::{DeserializeSnbt, functions::*},
            serialize::{SerializeSnbt, functions::*},
        },
    };
}
