#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(trivial_bounds)]

pub mod biomes;
pub mod blocks;
pub mod maps;
pub mod world;

mod plugin;
pub use plugin::WorldPlugin;
