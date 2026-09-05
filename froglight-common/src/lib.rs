#![doc = include_str!("../README.md")]
#![allow(clippy::disallowed_types, reason = "This crate selects and re-exports common types")]
#![cfg_attr(feature = "nightly", feature(core_float_math))]
#![cfg_attr(
    feature = "nightly",
    allow(unused_features, reason = "`core_float_math` used if `no_std`")
)]
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
            // pub use std::sync::{LazyLock, OnceLock};

            /// A re-export of [`std::sync::LazyLock`].
            pub type LazyLock<T, F = fn() -> T> = std::sync::LazyLock<T, F>;
            /// A re-export of [`std::sync::OnceLock`].
            pub type OnceLock<T> = std::sync::OnceLock<T>;
        }
        all(feature = "once_cell", feature = "critical-section") => {
            // pub use ::once_cell::sync::{Lazy as LazyLock, OnceCell as OnceLock};

            /// A re-export of [`once_cell::sync::Lazy`].
            pub type LazyLock<T, F = fn() -> T> = ::once_cell::sync::Lazy<T, F>;
            /// A re-export of [`once_cell::sync::OnceCell`].
            pub type OnceLock<T> = ::once_cell::sync::OnceCell<T>;
        }
        _ => {}
    }

    // Prefer `parking_lot` Mutex/RwLock over `std`
    cfg_select! {
        feature = "parking_lot" => {
            // pub use ::parking_lot::{Mutex, RwLock};

            /// A re-export of [`parking_lot::Mutex`].
            pub type Mutex<T> = ::parking_lot::Mutex<T>;
            /// A re-export of [`parking_lot::RwLock`].
            pub type RwLock<T> = ::parking_lot::RwLock<T>;
        }
        all(feature = "std", feature = "nightly") => {
            // pub use std::sync::nonpoison::{Mutex, RwLock};

            /// A re-export of [`std::sync::nonpoison::Mutex`].
            pub type Mutex<T> = std::sync::nonpoison::Mutex<T>;
            /// A re-export of [`std::sync::nonpoison::RwLock`].
            pub type RwLock<T> = std::sync::nonpoison::RwLock<T>;
        }
        feature = "std" => {
            // pub use std::sync::{Mutex, RwLock};

            /// A re-export of [`std::sync::Mutex`].
            pub type Mutex<T> = std::sync::Mutex<T>;
            /// A re-export of [`std::sync::RwLock`].
            pub type RwLock<T> = std::sync::RwLock<T>;
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
