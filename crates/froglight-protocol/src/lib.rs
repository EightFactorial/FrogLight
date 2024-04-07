#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(array_try_from_fn)]
#![feature(const_type_name)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod common;
pub mod protocol;
pub mod registries;
pub mod states;
pub mod traits;
pub mod versions;
