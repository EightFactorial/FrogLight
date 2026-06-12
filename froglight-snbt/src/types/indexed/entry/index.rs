//! TODO

use alloc::string::String;
use core::range::Range;

use crate::types::indexed::{
    index::{
        Index,
        bool::BoolDescription,
        numeric::{Float, FloatDescription, Integer, IntegerDescription},
        string::StringDescription,
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
    /// Create a new [`EntryIndex`] from the given name and value [`Index`]es.
    #[inline]
    #[must_use]
    pub const fn new(name: Index<String>, value: ValueIndex) -> Self { Self { name, value } }

    /// Get the [`Index`] of the name of this entry.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if this entry is nameless, or has a name
    /// [`Index`] of `0..0`.
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

/// A description of an SNBT value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueDescription {
    /// A value without a description.
    None,
    /// A [`bool`] description.
    Bool(BoolDescription),
    /// An [`Integer`] description.
    Int(IntegerDescription),
    /// A [`Float`] description.
    Float(FloatDescription),
    /// A [`String`] description.
    String(StringDescription),
}

impl ValueIndex {
    /// Get the inner [`Range`] of this [`ValueIndex`].
    #[must_use]
    pub const fn range(self) -> Range<usize> {
        match self {
            Self::Bool(i) => i.range(),
            Self::Byte(i) | Self::Short(i) | Self::Int(i) | Self::Long(i) => i.range(),
            Self::Float(i) | Self::Double(i) => i.range(),
            Self::String(i) => i.range(),
            Self::List(i) => i.range(),
            Self::Compound(i) => i.range(),
            Self::ByteArray(i) | Self::IntArray(i) | Self::LongArray(i) => i.range(),
        }
    }

    /// Get the [`ValueDescription`] of this [`ValueIndex`].
    #[must_use]
    pub const fn description(self) -> ValueDescription {
        match self {
            Self::Bool(i) => ValueDescription::Bool(i.description()),
            Self::Byte(i) | Self::Short(i) | Self::Int(i) | Self::Long(i) => {
                ValueDescription::Int(i.description())
            }
            Self::Float(i) | Self::Double(i) => ValueDescription::Float(i.description()),
            Self::String(i) => ValueDescription::String(i.description()),
            _ => ValueDescription::None,
        }
    }
}
