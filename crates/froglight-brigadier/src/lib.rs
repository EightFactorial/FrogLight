#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(const_type_id)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod argument;
pub mod function;
pub mod graph;
pub mod plugin;

#[cfg(feature = "io")]
mod io;

#[cfg(test)]
mod test;

pub mod prelude {
    //! Re-exports of common types and traits.

    pub use crate::{
        argument::{ArgumentError, ArgumentParser, ReflectArgumentParser},
        graph::{AppBrigadierGraph, BrigadierError, BrigadierGraph},
        plugin::{
            BrigadierBuilder, BrigadierCommand, BrigadierCommands, BrigadierEvent, BrigadierPlugin,
        },
    };
}
