#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

pub mod instance;
pub mod plugin;
pub mod query;
pub mod queue;
pub mod relationship;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        instance::SessionInstance,
        query::{InInstance, OnInstance},
        queue::BlockEditQueue,
        relationship::PartOfInstance,
    };
}
