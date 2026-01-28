#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod core;
pub mod generated;
pub mod version;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        generated as packet,
        version::{PacketVersion, VersionPacket, VersionPacketDirectional},
    };
}
