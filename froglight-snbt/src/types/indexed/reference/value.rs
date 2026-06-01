use alloc::string::String;
use core::{fmt, marker::PhantomData};

// use froglight_nbt::types::indexed::types::{IndexedListType, IndexedMapType};
use crate::types::indexed::{
    compound::IndexedCompound, core::IndexCore, entry::ValueIndex, reference::IndexedReference,
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
    ByteArray(PhantomData<C>),
    /// A [`String`] value.
    String(IndexedReference<'data, String>),
    /// A list of values.
    List(PhantomData<C>),
    /// A compound of named entries.
    Compound(IndexedCompound<'data, C>),
    /// A slice of [`u32`] values.
    IntArray(PhantomData<C>),
    /// A slice of [`u64`] values.
    LongArray(PhantomData<C>),
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

            ValueIndex::List(_index) => todo!(),
            ValueIndex::Compound(_index) => todo!(),

            ValueIndex::ByteArray(_index) => todo!(),
            ValueIndex::IntArray(_index) => todo!(),
            ValueIndex::LongArray(_index) => todo!(),
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
    #[must_use]
    /// Return a reference to the stored value if it is of type
    /// [`IndexedCompound`], else `None`.
    pub const fn as_compound(self) -> Option<IndexedCompound<'data, C>> {
        if let Self::Compound(value) = self { Some(value) } else { None }
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

            Self::List(_) => f.debug_tuple("List").finish_non_exhaustive(),
            Self::Compound(value) => f.debug_tuple("Compound").field(value).finish(),

            Self::ByteArray(_) => f.debug_tuple("ByteArray").finish_non_exhaustive(),
            Self::IntArray(_) => f.debug_tuple("IntArray").finish_non_exhaustive(),
            Self::LongArray(_) => f.debug_tuple("LongArray").finish_non_exhaustive(),
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

            (Self::List(_), Self::List(_)) => todo!(),
            (Self::Compound(a), Self::Compound(b)) => a == b,

            (Self::ByteArray(_), Self::ByteArray(_)) => todo!(),
            (Self::IntArray(_), Self::IntArray(_)) => todo!(),
            (Self::LongArray(_), Self::LongArray(_)) => todo!(),

            _ => false,
        }
    }
}
impl<C: IndexCore> Eq for ValueReference<'_, C> {}
