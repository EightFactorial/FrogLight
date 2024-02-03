#![doc = include_str!("../README.md")]

pub mod events;
pub mod resources;
pub mod systemsets;

// Re-export big_space
pub use big_space;

mod plugin;
pub use plugin::CorePlugin;
