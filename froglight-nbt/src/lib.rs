#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "facet")]
pub mod facet;
pub mod types;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "froglight-facet")]
    pub use crate::facet::froglight_facet::NbtTemplate;
    pub use crate::types::{
        indexed::{
            IndexedNbt,
            core::{IndexedNbtCow, IndexedNbtSlice},
        },
        structured::Nbt,
    };
    #[cfg(feature = "facet")]
    pub use crate::{
        self as nbt,
        facet::{
            deserialize::{DeserializeNbt, functions::*},
            serialize::{SerializeNbt, functions::*},
        },
    };
}
