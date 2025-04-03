#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod schedule;

#[cfg(feature = "subapp")]
pub mod subapp;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.
}
