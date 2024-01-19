#![doc = include_str!("../README.md")]

pub mod events;
pub mod resources;
pub mod systemsets;

mod plugin;
pub use plugin::CorePlugin;
