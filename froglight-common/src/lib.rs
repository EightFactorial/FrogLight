#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(nonpoison_mutex, nonpoison_rwlock))]
#![cfg_attr(feature = "nightly", allow(unused_features, reason = "Used if no `parking_lot`"))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod identifier;
pub mod impossible;
pub mod lpdvec3;
pub mod version;

pub mod crates {
    //! Re-exports of common crates and dependencies.

    #[cfg(feature = "critical-section")]
    pub use ::critical_section;
    #[cfg(feature = "indexmap")]
    pub use ::foldhash;
    #[cfg(feature = "glam")]
    pub use ::glam;
    #[cfg(feature = "indexmap")]
    pub use ::indexmap;
    #[cfg(feature = "libm")]
    pub use ::libm;
    #[cfg(feature = "once_cell")]
    pub use ::once_cell;
    #[cfg(feature = "parking_lot")]
    pub use ::parking_lot;
    #[cfg(feature = "serde")]
    pub use ::serde;
    #[cfg(feature = "uuid")]
    pub use ::uuid;
}

pub mod types {
    //! Re-exports of common types based on enabled features.

    // Prefer `std` Lazy/Once over `once_cell`
    cfg_select! {
        feature = "std" => {
            pub use std::sync::{LazyLock, OnceLock};
        }
        all(feature = "once_cell", feature = "critical-section") => {
            pub use ::once_cell::sync::{Lazy as LazyLock, OnceCell as OnceLock};
        }
        _ => {}
    }

    // Prefer `parking_lot` Mutex/RwLock over `std`
    cfg_select! {
        feature = "parking_lot" => {
            pub use ::parking_lot::{Mutex, RwLock};
        }
        all(feature = "std", feature = "nightly") => {
            pub use std::sync::nonpoison::{Mutex, RwLock};
        }
        feature = "std" => {
            pub use std::sync::{Mutex, RwLock};
        }
        _ => {}
    }

    /// A type alias for an [`IndexMap`](indexmap::IndexMap) using
    /// [`foldhash::fast::RandomState`] as the hasher.
    #[cfg(feature = "indexmap")]
    pub type IndexMap<K, V, S = foldhash::fast::RandomState> = ::indexmap::IndexMap<K, V, S>;
}

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "alloc")]
    pub use crate::identifier::Identifier;
    pub use crate::{identifier::Ident, version::*};
}
