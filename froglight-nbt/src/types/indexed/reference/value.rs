use core::fmt;

use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, NbtAccess, Ref},
    index::Index,
    list::IndexedValueList,
    reference::{IndexableValue, IndexableValueMut, IndexedReference},
};

macro_rules! impl_indexable {
    ($($ty:ty),*) => {
        $(
            unsafe impl IndexableValue for $ty {
                type Value<'a> = Self;

                const LIST_INDEX_IS_ENTRY_RANGE: bool = false;

                unsafe fn size(_: &[u8], _: Index<Self>) -> usize {
                    core::mem::size_of::<Self>()
                }

                unsafe fn get(slice: &[u8], index: Index<Self>) -> Self::Value<'_> {
                    unsafe {
                        let ptr = slice.as_ptr().add(index.value());
                        let val = core::ptr::read_unaligned(ptr.cast::<Self>());
                        Self::from_ne_bytes(val.to_be_bytes())
                    }
                }
            }

            impl IndexableValueMut for $ty {
                unsafe fn set(slice: &mut [u8], index: Index<Self>, value: Self::Value<'_>) {
                    unsafe {
                        let ptr = slice.as_mut_ptr().add(index.value());
                        let val = Self::from_ne_bytes(value.to_be_bytes());
                        core::ptr::write_unaligned(ptr.cast::<Self>(), val);
                    }
                }
            }
        )*
    };
}

impl_indexable!(u8, u16, u32, u64, f32, f64);

// -------------------------------------------------------------------------------------------------

/// A reference to an NBT value that is indexed by an [`IndexCore`].
pub enum IndexedValueReference<'data, A: NbtAccess, C: IndexCore<A> + 'data> {
    /// A [`u8`] value.
    Byte(IndexedReference<'data, u8, A>),
    /// A [`u16`] value.
    Short(IndexedReference<'data, u16, A>),
    /// A [`u32`] value.
    Int(IndexedReference<'data, u32, A>),
    /// A [`u64`] value.
    Long(IndexedReference<'data, u64, A>),
    /// A [`f32`] value.
    Float(IndexedReference<'data, f32, A>),
    /// A [`f64`] value.
    Double(IndexedReference<'data, f64, A>),
    /// A [`u8`] array.
    ByteArray(IndexedReference<'data, [u8], A>),
    /// An [`MStr`] string.
    String(IndexedReference<'data, MStr, A>),
    /// A list of values.
    List(IndexedValueList<'data, A, C>),
    /// A compound of named entries.
    Compound(IndexedCompound<'data, A, C>),
    /// A [`u32`] array.
    IntArray(IndexedReference<'data, [u32], A>),
    /// A [`u64`] array.
    LongArray(IndexedReference<'data, [u64], A>),
}

macro_rules! create_fns {
    ($($ident:ident: $ty:ty => $variant:ident),*) => {
        impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> IndexedValueReference<'data, A, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a reference to the stored reference if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $ident(self) -> Option<$ty> {
                    if let IndexedValueReference::$variant(value) = self {
                        Some(value)
                    } else {
                        None
                    }
                }
            )*
        }

        impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> fmt::Debug
            for IndexedValueReference<'_, A, C>
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        IndexedValueReference::$variant(value) => f.debug_tuple(stringify!($variant)).field(value).finish(),
                    )*
                }
            }
        }

        impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> Clone for IndexedValueReference<'data, A, C>
        where
            <A as NbtAccess>::CORE<'data, C>: Clone,
            <A as NbtAccess>::SLICE<'data>: Clone,
        {
            fn clone(&self) -> Self {
                match self {
                    $(
                        IndexedValueReference::$variant(value) => IndexedValueReference::$variant(value.clone()),
                    )*
                }
            }
        }
        impl<'data, A: NbtAccess, C: IndexCore<A> + 'data> Copy for IndexedValueReference<'data, A, C>
        where
            <A as NbtAccess>::CORE<'data, C>: Copy,
            <A as NbtAccess>::SLICE<'data>: Copy,
        {
        }

        impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> PartialEq
            for IndexedValueReference<'data, A, C>
        where
            A::SLICE<'data>: PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        (IndexedValueReference::$variant(a), IndexedValueReference::$variant(b)) => a == b,
                    )*
                    _ => false,
                }
            }
        }
        impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> Eq
            for IndexedValueReference<'data, A, C>
        where
            A::SLICE<'data>: Eq,
        {
        }

        $(
            impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> From<$ty>
                for IndexedValueReference<'data, A, C>
            {
                fn from(value: $ty) -> Self {
                    IndexedValueReference::$variant(value.into())
                }
            }

            impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> TryFrom<IndexedValueReference<'data, A, C>> for $ty {
                type Error = IndexedValueReference<'data, A, C>;

                fn try_from(value: IndexedValueReference<'data, A, C>) -> Result<Self, Self::Error> {
                    if let IndexedValueReference::$variant(inner) = value {
                        Ok(inner)
                    } else {
                        Err(value)
                    }
                }
            }
        )*
    };
}

create_fns! {
    as_byte: IndexedReference<'data, u8, A> => Byte,
    as_short: IndexedReference<'data, u16, A> => Short,
    as_int: IndexedReference<'data, u32, A> => Int,
    as_long: IndexedReference<'data, u64, A> => Long,
    as_float: IndexedReference<'data, f32, A> => Float,
    as_double: IndexedReference<'data, f64, A> => Double,
    as_byte_array: IndexedReference<'data, [u8], A> => ByteArray,
    as_string: IndexedReference<'data, MStr, A> => String,
    as_list: IndexedValueList<'data, A, C> => List,
    as_compound: IndexedCompound<'data, A, C> => Compound,
    as_int_array: IndexedReference<'data, [u32], A> => IntArray,
    as_long_array: IndexedReference<'data, [u64], A> => LongArray
}
