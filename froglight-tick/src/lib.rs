#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod bevy;
pub mod disable;
pub mod schedule;
pub mod timer;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        disable::TickDisabled,
        schedule::TickSchedule::{self, PostTick, PreTick, Tick, TickFirst, TickLast},
        timer::TickTimer,
    };
}
