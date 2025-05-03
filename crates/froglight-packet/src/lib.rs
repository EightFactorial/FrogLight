#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod common;
pub mod state;

pub mod v1_21_4;
pub mod v1_21_5;

pub mod prelude {
    //! Re-exports of commonly used types, traits, and macros.

    pub use crate::{
        common::{ConnectionIntent, ServerStatus},
        state::{Config, Handshake, Login, Play, Status},
    };
}
