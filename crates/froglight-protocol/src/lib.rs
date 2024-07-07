//! # Froglight Protocol
//! A barebones implementation of reading and writing types and packets in the
//! Minecraft format.
//!
//! This crate contains no networking logic, only logic for
//! reading and writing bytes and data structures!
//!
//! Most likely you are looking for the [Froglight
//! Network](../froglight-network/) crate, which implements a  basic
//! [`Connection`](../froglight-network/src/connection/mod.rs) using this
//! protocol.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(array_try_from_fn)]
#![feature(const_type_name)]
#![feature(trivial_bounds)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(clippy::struct_excessive_bools)]

pub mod common;
pub mod packet;
pub mod protocol;
pub mod states;
pub mod traits;
pub mod versions;
