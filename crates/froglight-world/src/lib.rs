#![doc = include_str!("../README.md")]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(array_try_from_fn)]

mod blocks;
pub use blocks::*;

mod map;
pub use map::*;

mod plugin;
pub use plugin::WorldPlugin;

pub mod world;
