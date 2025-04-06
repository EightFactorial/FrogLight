#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod schedule;
pub mod systemset;
pub mod tick;

#[cfg(feature = "subapp")]
pub mod subapp;

pub mod prelude {
    //! Re-exports of common types and traits.

    pub use crate::{
        schedule::{PostNetwork, PostTick, PreNetwork, PreTick, Tick},
        tick::{CurrentTick, ShouldTick, TickRate},
    };
}
