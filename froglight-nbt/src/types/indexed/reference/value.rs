use core::fmt;

use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, NbtAccess, Ref},
    index::Index,
    list::ValueList,
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
pub enum ValueReference<'index, A: NbtAccess, C: IndexCore<A> + 'index> {
    /// A [`u8`] value.
    Byte(IndexedReference<'index, u8, A>),
    /// A [`u16`] value.
    Short(IndexedReference<'index, u16, A>),
    /// A [`u32`] value.
    Int(IndexedReference<'index, u32, A>),
    /// A [`u64`] value.
    Long(IndexedReference<'index, u64, A>),
    /// A [`f32`] value.
    Float(IndexedReference<'index, f32, A>),
    /// A [`f64`] value.
    Double(IndexedReference<'index, f64, A>),
    /// A [`u8`] array.
    ByteArray(IndexedReference<'index, [u8], A>),
    /// An [`MStr`] string.
    String(IndexedReference<'index, MStr, A>),
    /// A list of values.
    List(ValueList<'index, A, C>),
    /// A compound of named entries.
    Compound(IndexedCompound<'index, A, C>),
    /// A [`u32`] array.
    IntArray(IndexedReference<'index, [u32], A>),
    /// A [`u64`] array.
    LongArray(IndexedReference<'index, [u64], A>),
}

macro_rules! create_fns {
    ($($ident:ident: $ty:ty => $variant:ident),*) => {
        impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> ValueReference<'index, A, C> {
            $(
                #[must_use]
                #[doc = concat!("Return a reference to the stored reference if it is of type [`", stringify!($ty), "`], else `None`.")]
                pub fn $ident(self) -> Option<$ty> {
                    if let ValueReference::$variant(value) = self {
                        Some(value)
                    } else {
                        None
                    }
                }
            )*
        }

        impl<A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> fmt::Debug
            for ValueReference<'_, A, C>
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        ValueReference::$variant(value) => f.debug_tuple(stringify!($variant)).field(value).finish(),
                    )*
                }
            }
        }

        impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> Clone for ValueReference<'index, A, C>
        where
            <A as NbtAccess>::CORE<'index, C>: Clone,
            <A as NbtAccess>::SLICE<'index>: Clone,
        {
            fn clone(&self) -> Self {
                match self {
                    $(
                        ValueReference::$variant(value) => ValueReference::$variant(value.clone()),
                    )*
                }
            }
        }
        impl<'index, A: NbtAccess, C: IndexCore<A> + 'index> Copy for ValueReference<'index, A, C>
        where
            <A as NbtAccess>::CORE<'index, C>: Copy,
            <A as NbtAccess>::SLICE<'index>: Copy,
        {
        }

        impl<'index, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> PartialEq
            for ValueReference<'index, A, C>
        where
            A::SLICE<'index>: PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        (ValueReference::$variant(a), ValueReference::$variant(b)) => a == b,
                    )*
                    _ => false,
                }
            }
        }
        impl<'index, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> Eq
            for ValueReference<'index, A, C>
        where
            A::SLICE<'index>: Eq,
        {
        }

        $(
            impl<'index, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> From<$ty>
                for ValueReference<'index, A, C>
            {
                fn from(value: $ty) -> Self {
                    ValueReference::$variant(value.into())
                }
            }

            impl<'index, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A>> TryFrom<ValueReference<'index, A, C>> for $ty {
                type Error = ValueReference<'index, A, C>;

                fn try_from(value: ValueReference<'index, A, C>) -> Result<Self, Self::Error> {
                    if let ValueReference::$variant(inner) = value {
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
    as_byte: IndexedReference<'index, u8, A> => Byte,
    as_short: IndexedReference<'index, u16, A> => Short,
    as_int: IndexedReference<'index, u32, A> => Int,
    as_long: IndexedReference<'index, u64, A> => Long,
    as_float: IndexedReference<'index, f32, A> => Float,
    as_double: IndexedReference<'index, f64, A> => Double,
    as_byte_array: IndexedReference<'index, [u8], A> => ByteArray,
    as_string: IndexedReference<'index, MStr, A> => String,
    as_list: ValueList<'index, A, C> => List,
    as_compound: IndexedCompound<'index, A, C> => Compound,
    as_int_array: IndexedReference<'index, [u32], A> => IntArray,
    as_long_array: IndexedReference<'index, [u64], A> => LongArray
}
