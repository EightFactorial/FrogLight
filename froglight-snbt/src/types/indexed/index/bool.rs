//! TODO

use bitflags::bitflags;

use crate::types::indexed::index::{Index, Indexable, IndexableValue, numeric::IntegerValue};

impl Indexable for bool {
    type Description = ();
}

/// A description of a boolean value.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolDescription(BooleanFlags);

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    struct BooleanFlags: u8 {
    }
}

impl BoolDescription {}

// -------------------------------------------------------------------------------------------------

/// A boolean value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BooleanValue {
    /// A [`bool`] value.
    Bool(bool),
    /// An [`IntegerValue`].
    Integer(IntegerValue),
}

impl BooleanValue {
    /// Get this [`BooleanValue`] as a [`bool`].
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::Bool(v) => v,
            Self::Integer(IntegerValue::Byte(v)) => v != 0,
            Self::Integer(IntegerValue::Short(v)) => v != 0,
            Self::Integer(IntegerValue::Int(v)) => v != 0,
            Self::Integer(IntegerValue::Long(v)) => v != 0,
        }
    }
}

impl IndexableValue for bool {
    type Value<'a> = BooleanValue;

    unsafe fn read_value(_: Index<Self>, _: &str) -> Self::Value<'_> { todo!() }
}
