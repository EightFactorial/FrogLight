#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(array_try_from_fn))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod standard;
pub mod variable;
pub mod version;

#[cfg(feature = "serde")]
pub mod serde;

pub mod prelude {
    //! Re-exports of all protocol traits, error types, and macros.

    pub use froglight_macros::FrogBuf;

    #[cfg(feature = "serde")]
    pub use crate::serde::FrogJson;
    pub use crate::{
        standard::{FrogRead, FrogWrite, ReadError, WriteError},
        variable::{FrogVarRead, FrogVarWrite},
    };
}
