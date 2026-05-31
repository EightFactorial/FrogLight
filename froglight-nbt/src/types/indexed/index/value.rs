//! TODO

use core::cmp::Ordering;

use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    index::Index,
    types::{IndexedListType, IndexedMapType},
};

/// The [`Index`] of an NBT value.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ValueIndex {
    /// A [`u8`] value.
    Byte(Index<u8>),
    /// A [`u16`] value.
    Short(Index<u16>),
    /// A [`u32`] value.
    Int(Index<u32>),
    /// A [`u64`] value.
    Long(Index<u64>),
    /// A [`f32`] value.
    Float(Index<f32>),
    /// A [`f64`] value.
    Double(Index<f64>),
    /// A [`u8`] array.
    ByteArray(Index<[u8]>),
    /// An [`MStr`] string.
    String(Index<MStr>),
    /// A list of values.
    List(Index<IndexedListType>),
    /// A compound of named entries.
    Compound(Index<IndexedMapType>),
    /// A [`u32`] array.
    IntArray(Index<[u32]>),
    /// A [`u64`] array.
    LongArray(Index<[u64]>),
}

impl ValueIndex {
    /// Get the raw index value.
    #[inline]
    #[must_use]
    pub const fn index(&self) -> usize {
        match self {
            Self::Byte(index) => index.value(),
            Self::Short(index) => index.value(),
            Self::Int(index) => index.value(),
            Self::Long(index) => index.value(),
            Self::Float(index) => index.value(),
            Self::Double(index) => index.value(),
            Self::ByteArray(index) => index.value(),
            Self::String(index) => index.value(),
            Self::List(index) => index.value(),
            Self::Compound(index) => index.value(),
            Self::IntArray(index) => index.value(),
            Self::LongArray(index) => index.value(),
        }
    }
}

impl PartialOrd for ValueIndex {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for ValueIndex {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering { self.index().cmp(&other.index()) }
}
