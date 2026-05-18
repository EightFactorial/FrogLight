use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    index::Index,
    reference::{IndexableSlice, IndexableValue},
};

macro_rules! impl_indexable {
    ($([$ty:ty]: $N:literal),*) => {
        $(
            unsafe impl IndexableValue for [$ty] {
                type Value<'slice> = &'slice Self;

                const LIST_INDEX_IS_ENTRY_RANGE: bool = true;

                unsafe fn size(slice: &[u8], index: Index<Self>) -> usize {
                    unsafe { Self::slice_len(slice, index) + Self::SIZE_BYTES }
                }

                unsafe fn get(slice: &[u8], index: Index<Self>) -> Self::Value<'_>
                {
                    match core::alloc::Layout::new::<$ty>().align() {
                        1 => {
                            unsafe {
                                let len = Self::slice_len(slice, index);
                                debug_assert!(index.value() + Self::SIZE_BYTES + len <= slice.len());

                                let ptr = slice.as_ptr().add(index.value()).add(Self::SIZE_BYTES);
                                core::slice::from_raw_parts(ptr.cast(), len)
                            }
                        }
                        _ => unreachable!("Only `u8` slices can be retrieved as values!"),
                    }
                }
            }

            impl IndexableSlice for [$ty] {
                const SIZE_BYTES: usize = $N;

                unsafe fn slice_len(slice: &[u8], index: Index<Self>) -> usize {
                    debug_assert!(index.value() + Self::SIZE_BYTES <= slice.len());

                    unsafe {
                        let ptr = slice.as_ptr().add(index.value());
                        match Self::SIZE_BYTES {
                            2 => {
                                let val = ptr.cast::<u16>().read_unaligned();
                                usize::from(val.to_be())
                            }
                            4 => {
                                let val = ptr.cast::<u32>().read_unaligned();
                                usize::try_from(val.to_be()).expect("Length exceeds usize::MAX!")
                            }
                            8 => {
                                let val = ptr.cast::<u64>().read_unaligned();
                                usize::try_from(val.to_be()).expect("Length exceeds usize::MAX!")
                            }
                            _ => unreachable!("Only 2, 4, or 8-byte size prefixes are supported!"),
                        }
                    }
                }
            }
        )*
    };
}

impl_indexable!([u8]: 4, [u32]: 4, [u64]: 4);

// -------------------------------------------------------------------------------------------------

unsafe impl IndexableValue for MStr {
    type Value<'slice> = &'slice Self;

    const LIST_INDEX_IS_ENTRY_RANGE: bool = true;

    unsafe fn size(slice: &[u8], index: Index<Self>) -> usize {
        unsafe { Self::slice_len(slice, index) + Self::SIZE_BYTES }
    }

    unsafe fn get(slice: &[u8], index: Index<Self>) -> Self::Value<'_> {
        unsafe {
            let len = Self::slice_len(slice, index);
            debug_assert!(index.value() + Self::SIZE_BYTES + len <= slice.len());

            let ptr = slice.as_ptr().add(index.value()).add(Self::SIZE_BYTES);
            let slice = core::slice::from_raw_parts(ptr, len);
            MStr::from_mutf8_unchecked(slice)
        }
    }
}
impl IndexableSlice for MStr {
    const SIZE_BYTES: usize = 2;

    unsafe fn slice_len(slice: &[u8], index: Index<Self>) -> usize {
        debug_assert_ne!(index.value(), 0, "Got the name of something unnamed!");
        debug_assert!(index.value() + Self::SIZE_BYTES <= slice.len());

        unsafe {
            let ptr = slice.as_ptr().add(index.value());
            let val = ptr.cast::<u16>().read_unaligned();
            usize::from(val.to_be())
        }
    }
}
