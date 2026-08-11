//! TODO

use core::fmt;

use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    core::{IndexCore, Mut, NbtAccess, Ref},
    list::{IndexedList, iter::ListValueIter},
    reference::ValueReference,
    types::{IndexedListType, IndexedMapType},
};

/// An NBT List that is indexed by an [`IndexCore`].
pub enum ValueList<'data, A: NbtAccess, C: IndexCore<A> + 'data> {
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

impl<'data, A: NbtAccess, C: IndexCore<A>> ValueList<'data, A, C> {
    /// Returns a [`ValueReference`](ValueReference) to the value at the given
    /// index, if it exists.
    #[must_use]
    pub fn get(self, index: usize) -> Option<ValueReference<'data, A, C>> {
        match self {
            Self::Empty => None,
            Self::Byte(list) => list.get_indexed(index).map(ValueReference::Byte),
            Self::Short(list) => list.get_indexed(index).map(ValueReference::Short),
            Self::Int(list) => list.get_indexed(index).map(ValueReference::Int),
            Self::Long(list) => list.get_indexed(index).map(ValueReference::Long),
            Self::Float(list) => list.get_indexed(index).map(ValueReference::Float),
            Self::Double(list) => list.get_indexed(index).map(ValueReference::Double),
            Self::ByteArray(list) => list.get_indexed(index).map(ValueReference::ByteArray),
            Self::String(list) => list.get_indexed(index).map(ValueReference::String),
            Self::IntArray(list) => list.get_indexed(index).map(ValueReference::IntArray),
            Self::LongArray(list) => list.get_indexed(index).map(ValueReference::LongArray),
            Self::List(list) => list.get(index).map(ValueReference::List),
            Self::Compound(list) => list.get(index).map(ValueReference::Compound),
        }
    }

    /// Returns a [`ValueReference`](ValueReference) to the value at the given
    /// index, if it exists.
    #[must_use]
    pub fn get_ref(&self, index: usize) -> Option<ValueReference<'_, Ref, C>>
    where
        C: IndexCore<Ref>,
    {
        match self {
            Self::Empty => None,
            Self::Byte(list) => list.get_indexed_ref(index).map(ValueReference::Byte),
            Self::Short(list) => list.get_indexed_ref(index).map(ValueReference::Short),
            Self::Int(list) => list.get_indexed_ref(index).map(ValueReference::Int),
            Self::Long(list) => list.get_indexed_ref(index).map(ValueReference::Long),
            Self::Float(list) => list.get_indexed_ref(index).map(ValueReference::Float),
            Self::Double(list) => list.get_indexed_ref(index).map(ValueReference::Double),
            Self::ByteArray(list) => list.get_indexed_ref(index).map(ValueReference::ByteArray),
            Self::String(list) => list.get_indexed_ref(index).map(ValueReference::String),
            Self::IntArray(list) => list.get_indexed_ref(index).map(ValueReference::IntArray),
            Self::LongArray(list) => list.get_indexed_ref(index).map(ValueReference::LongArray),
            Self::List(list) => list.get_ref(index).map(ValueReference::List),
            Self::Compound(list) => list.get_ref(index).map(ValueReference::Compound),
        }
    }

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
            Self::List(list) => list.len(),
            Self::Compound(list) => list.len(),
        }
    }

    /// Returns `true` if the length of the list is zero.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Return an iterator over the values in this list.
    #[inline]
    #[must_use]
    pub const fn into_iter(self) -> ListValueIter<'data, A, C>
    where
        C: IndexCore<Ref>,
        ValueList<'data, A, C>: Copy,
    {
        ListValueIter::new(self)
    }

    /// Return an owned iterator over the entries in this compound.
    ///
    /// # Note
    ///
    /// This function requires `Copy`, but the iterator only requires `Clone`!
    ///
    /// This is to prevent accidental `Clone`s, which can be very expensive.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> ListValueIter<'data, A, C>
    where
        C: IndexCore<Ref>,
        ValueList<'data, A, C>: Copy,
    {
        ListValueIter::new(*self)
    }
}

impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> fmt::Debug for ValueList<'_, A, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueList::Empty => f.debug_tuple("Empty").finish(),
            ValueList::Byte(list) => f.debug_tuple("Byte").field(&list).finish(),
            ValueList::Short(list) => f.debug_tuple("Short").field(&list).finish(),
            ValueList::Int(list) => f.debug_tuple("Int").field(&list).finish(),
            ValueList::Long(list) => f.debug_tuple("Long").field(&list).finish(),
            ValueList::Float(list) => f.debug_tuple("Float").field(&list).finish(),
            ValueList::Double(list) => f.debug_tuple("Double").field(&list).finish(),
            ValueList::ByteArray(list) => f.debug_tuple("ByteArray").field(&list).finish(),
            ValueList::String(list) => f.debug_tuple("String").field(&list).finish(),
            ValueList::List(list) => f.debug_tuple("List").field(&list).finish(),
            ValueList::Compound(list) => f.debug_tuple("Compound").field(&list).finish(),
            ValueList::IntArray(list) => f.debug_tuple("IntArray").field(&list).finish(),
            ValueList::LongArray(list) => f.debug_tuple("LongArray").field(&list).finish(),
        }
    }
}

impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> Clone for ValueList<'data, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Clone,
    <A as NbtAccess>::SLICE<'data>: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Byte(list) => Self::Byte(list.clone()),
            Self::Short(list) => Self::Short(list.clone()),
            Self::Int(list) => Self::Int(list.clone()),
            Self::Long(list) => Self::Long(list.clone()),
            Self::Float(list) => Self::Float(list.clone()),
            Self::Double(list) => Self::Double(list.clone()),
            Self::ByteArray(list) => Self::ByteArray(list.clone()),
            Self::String(list) => Self::String(list.clone()),
            Self::IntArray(list) => Self::IntArray(list.clone()),
            Self::LongArray(list) => Self::LongArray(list.clone()),

            Self::List(list) => Self::List(list.clone()),
            Self::Compound(list) => Self::Compound(list.clone()),
        }
    }
}
impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> Copy for ValueList<'data, A, C>
where
    <A as NbtAccess>::CORE<'data, C>: Copy,
    <A as NbtAccess>::SLICE<'data>: Copy,
{
}

impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> PartialEq for ValueList<'data, A, C> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Byte(l0), Self::Byte(r0)) => l0 == r0,
            (Self::Short(l0), Self::Short(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Long(l0), Self::Long(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Double(l0), Self::Double(r0)) => l0 == r0,
            (Self::ByteArray(l0), Self::ByteArray(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Compound(l0), Self::Compound(r0)) => l0 == r0,
            (Self::IntArray(l0), Self::IntArray(r0)) => l0 == r0,
            (Self::LongArray(l0), Self::LongArray(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> Eq for ValueList<'data, A, C> {}

// -------------------------------------------------------------------------------------------------

macro_rules! create_fns {
    (@ref $($as_ident:ident & $into_ident:ident: $ty:ty => $variant:ident),*) => {
        impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> ValueList<'data, A, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a reference to the stored list if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $as_ident(&self) -> Option<&IndexedList<'data, $ty, A, C>> {
                    if let ValueList::$variant(value) = self {
                        Some(value)
                    } else {
                        None
                    }
                }

                #[must_use]
                #[doc = concat!("Return a the stored list if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $into_ident(self) -> Option<IndexedList<'data, $ty, A, C>> {
                    if let ValueList::$variant(value) = self {
                        Some(value)
                    } else {
                        None
                    }
                }
            )*
        }

        $(
            impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> From<IndexedList<'data, $ty, A, C>> for ValueList<'data, A, C> {
                fn from(value: IndexedList<'data, $ty, A, C>) -> Self {
                    ValueList::$variant(value)
                }
            }

            impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> TryFrom<ValueList<'data, A, C>> for IndexedList<'data, $ty, A, C> {
                type Error = ValueList<'data, A, C>;

                fn try_from(value: ValueList<'data, A, C>) -> Result<Self, Self::Error> {
                    if let ValueList::$variant(value) = value {
                        Ok(value)
                    } else {
                        Err(value)
                    }
                }
            }
        )*
    };
    (@mut $($ident:ident: $ty:ty => $variant:ident),*) => {
        impl<'data, C: IndexCore<Mut> + 'data> ValueList<'data, Mut, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a mutable reference to the stored value if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $ident(&mut self) -> Option<&mut IndexedList<'data, $ty, Mut, C>> {
                    if let ValueList::$variant(value) = self {
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
    as_byte & into_byte: u8 => Byte,
    as_short & into_short: u16 => Short,
    as_int & into_int: u32 => Int,
    as_long & into_long: u64 => Long,
    as_float & into_float: f32 => Float,
    as_double & into_double: f64 => Double,
    as_byte_array & into_byte_array: [u8] => ByteArray,
    as_string & into_string: MStr => String,
    as_int_array & into_int_array: [u32] => IntArray,
    as_long_array & into_long_array: [u64] => LongArray
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
