//! TODO

use alloc::vec::Vec;

/// A MUTF-8–encoded, growable string.
///
/// Equivalent to [`String`](alloc::string::String),
/// but uses MUTF-8 instead of UTF-8.
#[repr(transparent)]
pub struct MString(Vec<u8>);
