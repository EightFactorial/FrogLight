#![doc = include_str!("../README.md")]
#![no_std]

pub use froglight_block as block;
pub use froglight_common as common;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{block::prelude::*, common::prelude::*};
}
