#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod identifier;
pub mod impossible;
pub mod version;

pub mod crates {
    //! Re-exports of common crates and dependencies.

    #[cfg(feature = "critical-section")]
    pub use ::critical_section;
    pub use ::glam;
    #[cfg(feature = "libm")]
    pub use ::libm;
    #[cfg(feature = "once_cell")]
    pub use ::once_cell;
    #[cfg(feature = "parking_lot")]
    pub use ::parking_lot;
    #[cfg(feature = "serde")]
    pub use ::serde;
}

pub mod types {
    //! Re-exports of common types based on enabled features.

    // Prefer `std` Lazy/Once over `once_cell`
    cfg_select! {
        feature = "std" => {
            pub use std::sync::{LazyLock, OnceLock};
        }
        feature = "once_cell" => {
            pub use ::once_cell::sync::{Lazy as LazyLock, OnceCell as OnceLock};
        }
    }

    // Prefer `parking_lot` Mutex/RwLock over `std`
    cfg_select! {
        feature = "parking_lot" => {
            pub use parking_lot::{Mutex, RwLock};
        }
        feature = "std" => {
            pub use std::sync::{Mutex, RwLock};
        }
    }
}

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "alloc")]
    pub use crate::identifier::Identifier;
    pub use crate::{identifier::Ident, version::*};
}
