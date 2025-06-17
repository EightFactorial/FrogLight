#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", allow(incomplete_features), feature(generic_const_exprs))]
#![cfg_attr(feature = "nightly", feature(const_type_name))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::large_enum_variant)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod common;
pub mod multicast;
pub mod state;

pub mod v1_21_4;
pub mod v1_21_5;
pub mod v1_21_6;

pub mod prelude {
    //! Re-exports of commonly used types, traits, and macros.

    pub use crate::{
        common::{Axis, BlockPos, ChunkPos, ConnectionIntent, Direction, ServerStatus},
        state::{Config, Handshake, Login, Play, Status},
    };
}
