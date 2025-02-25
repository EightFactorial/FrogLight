#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_id)]

pub mod argument;
pub mod function;
pub mod graph;

pub mod prelude {
    //! Re-exports of common types and traits.

    pub use crate::{
        argument::{ArgumentError, ArgumentParser},
        graph::{AppBrigadierGraph, BrigadierError, BrigadierGraph},
    };
}
