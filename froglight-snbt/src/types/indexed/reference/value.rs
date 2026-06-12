use alloc::string::String;
use core::fmt;

// use froglight_nbt::types::indexed::types::{IndexedListType, IndexedMapType};
use crate::types::indexed::{
    compound::IndexedCompound,
    core::IndexCore,
    entry::{ValueDescription, ValueIndex},
    index::numeric::{FloatValue, IntegerValue},
    list::{IndexedList, IndexedSlice},
    reference::IndexedReference,
};

/// A reference to an SNBT value.
pub enum ValueReference<'data, C: IndexCore> {
    /// A [`bool`] value.
    Bool(IndexedReference<'data, bool>),
    /// A [`u8`] value.
    Byte(IndexedReference<'data, u8>),
    /// A [`u16`] value.
    Short(IndexedReference<'data, u16>),
    /// A [`u32`] value.
    Int(IndexedReference<'data, u32>),
    /// A [`u64`] value.
    Long(IndexedReference<'data, u64>),
    /// A [`f32`] value.
    Float(IndexedReference<'data, f32>),
    /// A [`f64`] value.
    Double(IndexedReference<'data, f64>),
    /// A slice of [`u8`] values.
    ByteArray(IndexedSlice<'data, C, u8>),
    /// A [`String`] value.
    String(IndexedReference<'data, String>),
    /// A list of values.
    List(IndexedList<'data, C>),
    /// A compound of named entries.
    Compound(IndexedCompound<'data, C>),
    /// A slice of [`u32`] values.
    IntArray(IndexedSlice<'data, C, u32>),
    /// A slice of [`u64`] values.
    LongArray(IndexedSlice<'data, C, u64>),
}

impl<'data, C: IndexCore> ValueReference<'data, C> {
    /// Create a new [`ValueReference`] from the given core and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure the index is valid for the given core.
    #[must_use]
    pub unsafe fn new(core: &'data C, value: ValueIndex) -> Self {
        match value {
            ValueIndex::Bool(index) => {
                Self::Bool(unsafe { IndexedReference::new(core.root(), index) })
            }
            ValueIndex::Byte(index) => {
                Self::Byte(unsafe { IndexedReference::new(core.root(), index) })
            }
            ValueIndex::Short(index) => {
                Self::Short(unsafe { IndexedReference::new(core.root(), index) })
            }
            ValueIndex::Int(index) => {
                Self::Int(unsafe { IndexedReference::new(core.root(), index) })
            }
            ValueIndex::Long(index) => {
                Self::Long(unsafe { IndexedReference::new(core.root(), index) })
            }
            ValueIndex::Float(index) => {
                Self::Float(unsafe { IndexedReference::new(core.root(), index) })
            }
            ValueIndex::Double(index) => {
                Self::Double(unsafe { IndexedReference::new(core.root(), index) })
            }
            ValueIndex::String(index) => {
                Self::String(unsafe { IndexedReference::new(core.root(), index) })
            }

            ValueIndex::List(index) => {
                // SAFETY: `Index<IndexedListType>` has a valid `IndexedList` range.
                Self::List(unsafe { IndexedList::new(core, index.range()) })
            }
            ValueIndex::Compound(index) => {
                // SAFETY: `Index<IndexedMapType>` has a valid `IndexedCompound` range.
                Self::Compound(unsafe { IndexedCompound::new(core, index.range()) })
            }

            ValueIndex::ByteArray(index) => {
                // SAFETY: `Index<IndexedSliceType<u8>>` has a valid `IndexedSlice<u8>` range.
                Self::ByteArray(unsafe { IndexedSlice::new(core, index.range()) })
            }
            ValueIndex::IntArray(index) => {
                // SAFETY: `Index<IndexedSliceType<u32>>` has a valid `IndexedSlice<u32>` range.
                Self::IntArray(unsafe { IndexedSlice::new(core, index.range()) })
            }
            ValueIndex::LongArray(index) => {
                // SAFETY: `Index<IndexedSliceType<u64>>` has a valid `IndexedSlice<u64>` range.
                Self::LongArray(unsafe { IndexedSlice::new(core, index.range()) })
            }
        }
    }

    /// Get a description of this value.
    #[must_use]
    pub const fn description(&self) -> ValueDescription {
        match self {
            Self::Bool(index) => ValueDescription::Bool(index.description()),
            Self::Byte(index) => ValueDescription::Int(index.description()),
            Self::Short(index) => ValueDescription::Int(index.description()),
            Self::Int(index) => ValueDescription::Int(index.description()),
            Self::Long(index) => ValueDescription::Int(index.description()),
            Self::Float(index) => ValueDescription::Float(index.description()),
            Self::Double(index) => ValueDescription::Float(index.description()),
            Self::String(index) => ValueDescription::String(index.description()),
            _ => ValueDescription::None,
        }
    }
}

macro_rules! create_fns {
    (
        $($ident:ident: $ty:ty => $variant:ident),*
    ) => {
        impl<'data, C: IndexCore> ValueReference<'data, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a reference to the stored value if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub const fn $ident(self) -> Option<IndexedReference<'data, $ty>> {
                    if let Self::$variant(value) = self {
                        Some(value)
                    } else {
                        None
                    }
                }
            )*
        }
    };
}

create_fns! {
    as_bool: bool => Bool,
    as_byte: u8 => Byte,
    as_short: u16 => Short,
    as_int: u32 => Int,
    as_long: u64 => Long,
    as_float: f32 => Float,
    as_double: f64 => Double,
    as_string: String => String
}

impl<'data, C: IndexCore> ValueReference<'data, C> {
    /// Return the stored value as a [`bool`] if it is of an integer or boolean
    /// type, else `None`.
    #[must_use]
    pub fn as_value_boolean(self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(value.get()),
            Self::Byte(value) => Some(value.get() != 0),
            Self::Short(value) => Some(value.get() != 0),
            Self::Int(value) => Some(value.get() != 0),
            Self::Long(value) => Some(value.get() != 0),
            _ => None,
        }
    }

    /// Return the stored value as an [`IntegerValue`] if it is of an integer or
    /// boolean type, else `None`.
    #[must_use]
    pub fn as_value_integer(self) -> Option<IntegerValue> {
        match self {
            Self::Bool(value) => Some(IntegerValue::Byte(u8::from(value.get()))),
            Self::Byte(value) => Some(IntegerValue::Byte(value.get())),
            Self::Short(value) => Some(IntegerValue::Short(value.get())),
            Self::Int(value) => Some(IntegerValue::Int(value.get())),
            Self::Long(value) => Some(IntegerValue::Long(value.get())),
            _ => None,
        }
    }

    /// Return the stored value as a [`FloatValue`] if it is of a float type,
    /// else `None`.
    #[must_use]
    pub fn as_value_float(self) -> Option<FloatValue> {
        match self {
            Self::Float(value) => Some(FloatValue::Float(value.get())),
            Self::Double(value) => Some(FloatValue::Double(value.get())),
            _ => None,
        }
    }

    /// Return a reference to the stored value if it is of type [`IndexedList`],
    /// else `None`.
    #[must_use]
    pub const fn as_list(self) -> Option<IndexedList<'data, C>> {
        if let Self::List(value) = self { Some(value) } else { None }
    }

    /// Return a reference to the stored value if it is of type
    /// [`IndexedCompound`], else `None`.
    #[must_use]
    pub const fn as_compound(self) -> Option<IndexedCompound<'data, C>> {
        if let Self::Compound(value) = self { Some(value) } else { None }
    }

    /// Return a reference to the stored value if it is of type [`IndexedSlice`]
    /// of [`u8`], else `None`.
    #[must_use]
    pub const fn as_byte_array(self) -> Option<IndexedSlice<'data, C, u8>> {
        if let Self::ByteArray(value) = self { Some(value) } else { None }
    }

    /// Return a reference to the stored value if it is of type [`IndexedSlice`]
    /// of [`u32`], else `None`.
    #[must_use]
    pub const fn as_int_array(self) -> Option<IndexedSlice<'data, C, u32>> {
        if let Self::IntArray(value) = self { Some(value) } else { None }
    }

    /// Return a reference to the stored value if it is of type [`IndexedSlice`]
    /// of [`u64`], else `None`.
    #[must_use]
    pub const fn as_long_array(self) -> Option<IndexedSlice<'data, C, u64>> {
        if let Self::LongArray(value) = self { Some(value) } else { None }
    }
}

// -------------------------------------------------------------------------------------------------

impl<C: IndexCore> fmt::Debug for ValueReference<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(value) => f.debug_tuple("Bool").field(value).finish(),
            Self::Byte(value) => f.debug_tuple("Byte").field(value).finish(),
            Self::Short(value) => f.debug_tuple("Short").field(value).finish(),
            Self::Int(value) => f.debug_tuple("Int").field(value).finish(),
            Self::Long(value) => f.debug_tuple("Long").field(value).finish(),
            Self::Float(value) => f.debug_tuple("Float").field(value).finish(),
            Self::Double(value) => f.debug_tuple("Double").field(value).finish(),
            Self::String(value) => f.debug_tuple("String").field(value).finish(),

            Self::List(value) => f.debug_tuple("List").field(value).finish(),
            Self::Compound(value) => f.debug_tuple("Compound").field(value).finish(),

            Self::ByteArray(value) => f.debug_tuple("ByteArray").field(value).finish(),
            Self::IntArray(value) => f.debug_tuple("IntArray").field(value).finish(),
            Self::LongArray(value) => f.debug_tuple("LongArray").field(value).finish(),
        }
    }
}

impl<C: IndexCore> Clone for ValueReference<'_, C> {
    #[inline]
    fn clone(&self) -> Self { *self }
}
impl<C: IndexCore> Copy for ValueReference<'_, C> {}

impl<C: IndexCore> PartialEq for ValueReference<'_, C> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Byte(a), Self::Byte(b)) => a == b,
            (Self::Short(a), Self::Short(b)) => a == b,
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Long(a), Self::Long(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::Double(a), Self::Double(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,

            (Self::List(a), Self::List(b)) => a == b,
            (Self::Compound(a), Self::Compound(b)) => a == b,

            (Self::ByteArray(a), Self::ByteArray(b)) => a == b,
            (Self::IntArray(a), Self::IntArray(b)) => a == b,
            (Self::LongArray(a), Self::LongArray(b)) => a == b,

            _ => false,
        }
    }
}
impl<C: IndexCore> Eq for ValueReference<'_, C> {}
