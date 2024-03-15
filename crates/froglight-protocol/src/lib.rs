#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(incomplete_features)]
#![feature(array_try_from_fn)]
#![feature(generic_const_exprs)]
#![feature(trivial_bounds)]

#[cfg(feature = "reflect")]
mod plugin;
#[cfg(feature = "reflect")]
pub use plugin::ReflectPlugin;

pub mod common;
pub mod io;
pub mod states;
pub mod traits;
pub mod versions;
