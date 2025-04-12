#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod schedule;
pub mod tick;

#[cfg(feature = "subapp")]
pub mod subapp;
pub mod subworld;

pub mod prelude {
    //! Re-exports of common types and traits.

    #[cfg(feature = "subapp")]
    pub use crate::subapp::{ReflectSubAppSync, SubAppSync};
    pub use crate::{
        schedule::label::*,
        subworld::{ReflectSubWorldSync, SubWorld, SubWorldSync, SubWorlds},
        tick::{CurrentTick, ShouldTick, TickRate},
    };
}
