//! TODO

use core::{fmt, range::Range};

use crate::types::indexed::index::{
    Index, Indexable, IndexableValue,
    numeric::{Integer, IntegerDescription, IntegerValue},
};

/// Whether a `bool(...)` operation was used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BooleanOperation {
    /// False
    False,
    /// True
    True,
}

impl Indexable for bool {
    type Description = BoolDescription;
}

/// A description of a boolean value.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoolDescription {
    /// A simple `true` or `false` value.
    Boolean,
    /// A `bool(...)` operation on an integer value.
    Integer(IntegerDescription),
}

impl BoolDescription {
    /// Create a new [`BoolDescription::Boolean`].
    #[inline]
    #[must_use]
    #[expect(clippy::new_without_default, reason = "No default value")]
    pub const fn new() -> Self { Self::Boolean }

    /// Create a new [`BoolDescription::Integer`].
    #[inline]
    #[must_use]
    pub const fn new_integer(desc: IntegerDescription) -> Self { Self::Integer(desc) }
}

impl fmt::Debug for BoolDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "Boolean"),
            Self::Integer(desc) => write!(f, "Integer({desc:?})"),
        }
    }
}

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

    unsafe fn read_value(index: Index<Self>, root: &str) -> Self::Value<'_> {
        match index.description() {
            BoolDescription::Boolean => {
                // SAFETY: The caller ensures that this is safe.
                let slice = unsafe { root.get_unchecked(index.range) };

                match slice {
                    "true" => BooleanValue::Bool(true),
                    "false" => BooleanValue::Bool(false),
                    #[cfg(debug_assertions)]
                    _ => panic!("Invalid boolean value: \"{slice}\""),
                    #[cfg(not(debug_assertions))]
                    _ => unsafe { core::hint::unreachable_unchecked() },
                }
            }
            BoolDescription::Integer(desc) => {
                // Shrink the range to exclude the `bool(` and `)` parts.
                debug_assert!(index.range.end - index.range.start >= 6);
                let range = Range { start: index.range.start + 5, end: index.range.end - 1 };

                // SAFETY: The caller and `Index` ensure that this is safe.
                unsafe { BooleanValue::Integer(Integer::read_value(Index::new(range, desc), root)) }
            }
        }
    }
}

impl From<bool> for BooleanValue {
    #[inline]
    fn from(value: bool) -> Self { Self::Bool(value) }
}
impl From<IntegerValue> for BooleanValue {
    #[inline]
    fn from(value: IntegerValue) -> Self { Self::Integer(value) }
}

impl From<BooleanValue> for bool {
    #[inline]
    fn from(value: BooleanValue) -> Self { value.as_bool() }
}
