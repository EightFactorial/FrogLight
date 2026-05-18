use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    compound::IndexedCompound,
    core::{IndexCore, NbtAccess},
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

cfg_select! {
    feature = "alloc" => {
        /// A reference to an NBT value that is indexed by an [`IndexCore`].
        pub enum IndexedValueReference<'data, A: NbtAccess, C: IndexCore<A> + 'data = crate::types::indexed::alloc::SliceCore<'data, A>> {
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
    }
    _ => {
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
    }
}
