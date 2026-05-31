//! TODO

use alloc::string::String;

use crate::types::indexed::{
    index::{
        Index,
        numeric::{Float, Integer},
    },
    types::{IndexedListType, IndexedMapType},
};

/// A pair of name and value [`Index`]es.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntryIndex {
    name: Index<String>,
    value: ValueIndex,
}

impl EntryIndex {
    /// Get the [`Index`] of the name of this entry.
    #[inline]
    #[must_use]
    pub const fn name(&self) -> Index<String> { self.name }

    /// Get the [`ValueIndex`] of this entry.
    #[inline]
    #[must_use]
    pub const fn value(&self) -> ValueIndex { self.value }
}

// -------------------------------------------------------------------------------------------------

/// The [`Index`] of an SNBT value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueIndex {
    /// A [`bool`] value.
    Bool(Index<bool>),
    /// A [`u8`] value.
    Byte(Index<Integer>),
    /// A [`u16`] value.
    Short(Index<Integer>),
    /// A [`u32`] value.
    Int(Index<Integer>),
    /// A [`u64`] value.
    Long(Index<Integer>),
    /// A [`f32`] value.
    Float(Index<Float>),
    /// A [`f64`] value.
    Double(Index<Float>),
    /// A slice of [`u8`] values.
    ByteArray(Index<[Integer]>),
    /// A [`String`] value.
    String(Index<String>),
    /// A list of values.
    List(Index<IndexedListType>),
    /// A compound of named entries.
    Compound(Index<IndexedMapType>),
    /// A slice of [`u32`] values.
    IntArray(Index<[Integer]>),
    /// A slice of [`u64`] values.
    LongArray(Index<[Integer]>),
}
