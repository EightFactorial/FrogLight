//! TODO

use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    core::{IndexCore, Mut, NbtAccess, Ref},
    list::IndexedList,
    types::{IndexedListType, IndexedMapType},
};

/// An NBT List that is indexed by an [`IndexCore`].
pub enum IndexedValueList<'data, A: NbtAccess, C: IndexCore<A> + 'data> {
    /// An empty list.
    Empty,
    /// A [`u8`] value.
    Byte(IndexedList<'data, u8, A, C>),
    /// A [`u16`] value.
    Short(IndexedList<'data, u16, A, C>),
    /// A [`u32`] value.
    Int(IndexedList<'data, u32, A, C>),
    /// A [`u64`] value.
    Long(IndexedList<'data, u64, A, C>),
    /// A [`f32`] value.
    Float(IndexedList<'data, f32, A, C>),
    /// A [`f64`] value.
    Double(IndexedList<'data, f64, A, C>),
    /// A [`u8`] array.
    ByteArray(IndexedList<'data, [u8], A, C>),
    /// An [`MStr`] string.
    String(IndexedList<'data, MStr, A, C>),
    /// A list of values.
    List(IndexedList<'data, IndexedListType, A, C>),
    /// A compound of named entries.
    Compound(IndexedList<'data, IndexedMapType, A, C>),
    /// A [`u32`] array.
    IntArray(IndexedList<'data, [u32], A, C>),
    /// A [`u64`] array.
    LongArray(IndexedList<'data, [u64], A, C>),
}

impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> IndexedValueList<'_, A, C> {
    /// Returns the length of this list.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Byte(list) => list.len(),
            Self::Short(list) => list.len(),
            Self::Int(list) => list.len(),
            Self::Long(list) => list.len(),
            Self::Float(list) => list.len(),
            Self::Double(list) => list.len(),
            Self::ByteArray(list) => list.len(),
            Self::String(list) => list.len(),
            Self::IntArray(list) => list.len(),
            Self::LongArray(list) => list.len(),

            Self::List(list) => unsafe {
                <C as IndexCore<A>>::entry_range(&list.core, list.index.value()).len()
            },
            Self::Compound(list) => unsafe {
                <C as IndexCore<A>>::entry_range(&list.core, list.index.value()).len()
            },
        }
    }

    /// Returns `true` if the length of the list is zero.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

// -------------------------------------------------------------------------------------------------

macro_rules! create_fns {
    (@ref $($ident:ident: $ty:ty => $variant:ident),*) => {
        impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedValueList<'data, A, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a reference to the stored list if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $ident(&self) -> Option<&IndexedList<'data, $ty, A, C>> {
                    if let IndexedValueList::$variant(value) = self {
                        Some(value)
                    } else {
                        None
                    }
                }
            )*
        }
    };
    (@mut $($ident:ident: $ty:ty => $variant:ident),*) => {
        impl<'data, C: IndexCore<Mut> + 'data> IndexedValueList<'data, Mut, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a mutable reference to the stored value if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $ident(&mut self) -> Option<&mut IndexedList<'data, $ty, Mut, C>> {
                    if let IndexedValueList::$variant(value) = self {
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
    @ref
    as_byte: u8 => Byte,
    as_short: u16 => Short,
    as_int: u32 => Int,
    as_long: u64 => Long,
    as_float: f32 => Float,
    as_double: f64 => Double,
    as_byte_array: [u8] => ByteArray,
    as_string: MStr => String,
    as_int_array: [u32] => IntArray,
    as_long_array: [u64] => LongArray
}
create_fns! {
    @mut
    as_byte_mut: u8 => Byte,
    as_short_mut: u16 => Short,
    as_int_mut: u32 => Int,
    as_long_mut: u64 => Long,
    as_float_mut: f32 => Float,
    as_double_mut: f64 => Double,
    as_byte_array_mut: [u8] => ByteArray,
    as_string_mut: MStr => String,
    as_int_array_mut: [u32] => IntArray,
    as_long_array_mut: [u64] => LongArray
}
