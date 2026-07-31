#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod bevy;
pub mod counter;
#[cfg(feature = "bevy_diagnostic")]
pub mod diagnostic;
pub mod disable;
pub mod event;
pub mod schedule;
pub mod timer;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        counter::TickCounter,
        disable::TickDisabled,
        event::Ticked,
        schedule::TickSchedule::{self, PostTick, PreTick, Tick, TickFirst, TickLast},
        timer::TickTimer,
    };
}
