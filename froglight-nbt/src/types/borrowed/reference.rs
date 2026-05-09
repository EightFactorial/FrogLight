//! TODO

use core::ops::Deref;

use froglight_mutf8::prelude::MStr;

use crate::types::borrowed::{
    core::{Mut, NbtAccess},
    index::Index,
};

/// A type that accessed via an [`Index`].
pub struct IndexedReference<'data, T: ?Sized, A: NbtAccess> {
    slice: A::SLICE<'data>,
    index: Index<T>,
}

impl<'data, T: ?Sized, A: NbtAccess> IndexedReference<'data, T, A> {
    /// Create a new [`IndexedReference`] from the given slice and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given slice.
    #[inline]
    #[must_use]
    pub const unsafe fn new(slice: A::SLICE<'data>, index: Index<T>) -> Self {
        Self { slice, index }
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for types that can be retrieved via an [`Index`].
pub trait NbtValueType: sealed::Sealed + Copy + Sized + 'static {
    /// Read a value of this type from the given slice and [`Index`].
    ///
    /// # Safety
    ///
    /// TODO
    #[must_use]
    unsafe fn read_unaligned(slice: &[u8], index: Index<Self>) -> Self;

    /// Write a value of this type to the given slice and [`Index`].
    ///
    /// # Safety
    ///
    /// TODO
    unsafe fn write_unaligned(&self, slice: &mut [u8], index: Index<Self>);
}

impl<T: NbtValueType, A: NbtAccess> IndexedReference<'_, T, A> {
    /// Get the value of this reference.
    #[inline]
    #[must_use]
    pub fn get(&self) -> T {
        // SAFETY: `IndexedReference` guarantees that the index is valid core.
        unsafe { T::read_unaligned(&self.slice, self.index) }
    }
}

impl<T: NbtValueType> IndexedReference<'_, T, Mut> {
    /// Set the value of this reference.
    #[inline]
    pub fn set(&mut self, value: T) {
        // SAFETY: `IndexedReference` guarantees that the index is valid core.
        unsafe { value.write_unaligned(self.slice, self.index) }
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for references that can be retrieved via an [`Index`].
pub trait NbtSliceType: sealed::Sealed + 'static {
    /// The number of bytes used to store the length of this type.
    const LENGTH_BYTES: usize;

    /// Read the length of this type from the given slice and [`Index`].
    ///
    /// # Safety
    ///
    /// TODO
    #[must_use]
    unsafe fn read_length(slice: &[u8], index: Index<Self>) -> usize;

    /// Create a reference to this type using the given slice and [`Index`].
    ///
    /// # Safety
    ///
    /// TODO
    #[must_use]
    unsafe fn create_reference(slice: &[u8], index: Index<Self>) -> &Self;
}

impl<T: NbtSliceType + ?Sized, A: NbtAccess> IndexedReference<'_, T, A> {
    /// Returns the number of elements in the slice.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        // SAFETY: `IndexedReference` guarantees that the index is valid core.
        unsafe { T::read_length(&self.slice, self.index) }
    }

    /// Returns true if the slice has a length of 0.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get a reference to the slice.
    #[inline]
    #[must_use]
    pub fn get_slice(&self) -> &T {
        // SAFETY: `IndexedReference` guarantees that the index is valid core.
        unsafe { T::create_reference(&self.slice, self.index) }
    }
}

impl<T: NbtSliceType + ?Sized, A: NbtAccess> Deref for IndexedReference<'_, T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target { self.get_slice() }
}
// -------------------------------------------------------------------------------------------------

macro_rules! impl_trait {
    (@value $ty:ty => { $($impl:tt)* }) => {
        impl NbtValueType for $ty {
            $($impl)*
        }
    };
    (@value $($ty:ty)+) => {
        $(
            impl_trait!(@value $ty =>
                {
                    #[inline]
                    #[allow(clippy::cast_ptr_alignment, reason = "Using `read_unaligned`")]
                    unsafe fn read_unaligned(slice: &[u8], index: Index<Self>) -> Self {
                        unsafe {
                            let ptr = slice.as_ptr().add(index.value()).cast::<Self>();
                            core::ptr::read_unaligned(ptr).to_be()
                        }
                    }

                    #[inline]
                    #[allow(clippy::cast_ptr_alignment, reason = "Using `write_unaligned`")]
                    unsafe fn write_unaligned(&self, slice: &mut [u8], index: Index<Self>) {
                        unsafe {
                            let ptr = slice.as_mut_ptr().add(index.value()).cast::<Self>();
                            core::ptr::write_unaligned(ptr, self.to_be());
                        }
                    }
                }
            );
        )+
    };

    (@slice $ty:ty => { $($impl:tt)* }) => {
        impl NbtSliceType for $ty {
            $($impl)*
        }
    };
}

impl_trait!(@value u8 u16 u32 u64);
impl_trait!(@value f32 => {
    #[inline]
    #[expect(clippy::cast_ptr_alignment, reason = "Using `read_unaligned`")]
    unsafe fn read_unaligned(slice: &[u8], index: Index<Self>) -> Self {
        unsafe {
            let ptr = slice.as_ptr().add(index.value()).cast::<Self>();
            f32::from_be_bytes(f32::to_ne_bytes(core::ptr::read_unaligned(ptr)))
        }
    }

    #[inline]
    #[expect(clippy::cast_ptr_alignment, reason = "Using `write_unaligned`")]
    unsafe fn write_unaligned(&self, slice: &mut [u8], index: Index<Self>) {
        let value = f32::from_be_bytes(f32::to_ne_bytes(*self));
        unsafe {
            let ptr = slice.as_mut_ptr().add(index.value()).cast::<Self>();
            core::ptr::write_unaligned(ptr, value);
        }
    }
});
impl_trait!(@value f64 => {
    #[inline]
    #[expect(clippy::cast_ptr_alignment, reason = "Using `read_unaligned`")]
    unsafe fn read_unaligned(slice: &[u8], index: Index<Self>) -> Self {
        unsafe {
            let ptr = slice.as_ptr().add(index.value()).cast::<Self>();
            f64::from_be_bytes(f64::to_ne_bytes(core::ptr::read_unaligned(ptr)))
        }
    }

    #[inline]
    #[expect(clippy::cast_ptr_alignment, reason = "Using `write_unaligned`")]
    unsafe fn write_unaligned(&self, slice: &mut [u8], index: Index<Self>) {
        let value = f64::from_be_bytes(f64::to_ne_bytes(*self));
        unsafe {
            let ptr = slice.as_mut_ptr().add(index.value()).cast::<Self>();
            core::ptr::write_unaligned(ptr, value);
        }
    }
});

impl_trait!(@slice [u8] => {
    const LENGTH_BYTES: usize = 4;

    #[inline]
    unsafe fn read_length(slice: &[u8], index: Index<Self>) -> usize {
        // SAFETY: The first 4 bytes of a slice are always the length of the slice.
        unsafe { NbtValueType::read_unaligned(slice, index.cast::<u32>()) as usize }
    }

    #[inline]
    unsafe fn create_reference(slice: &[u8], index: Index<Self>) -> &Self {
        // SAFETY: The first 4 bytes of a slice are always the length of the slice.
        unsafe {
            let length = Self::read_length(slice, index);
            let ptr = slice.as_ptr().add(index.value() + Self::LENGTH_BYTES);
            core::slice::from_raw_parts(ptr, length)
        }
    }
});
impl_trait!(@slice MStr => {
    const LENGTH_BYTES: usize = 2;

    #[inline]
    unsafe fn read_length(slice: &[u8], index: Index<Self>) -> usize {
        // SAFETY: The first 2 bytes of an MStr are always the length of the string.
        unsafe { NbtValueType::read_unaligned(slice, index.cast::<u16>()) as usize }
    }

    #[inline]
    unsafe fn create_reference(slice: &[u8], index: Index<Self>) -> &Self {
        // SAFETY: The first 2 bytes of an MStr are always the length of the string.
        // SAFETY: NBT strings are always valid MUTF-8.
        unsafe {
            let length = Self::read_length(slice, index);
            let ptr = slice.as_ptr().add(index.value() + Self::LENGTH_BYTES);
            let slice = core::slice::from_raw_parts(ptr, length);
            MStr::from_mutf8_unchecked(slice)
        }
    }
});

impl_trait!(@slice [u32] => {
    const LENGTH_BYTES: usize = 4;

    #[inline]
    unsafe fn read_length(slice: &[u8], index: Index<Self>) -> usize {
        // SAFETY: The first 4 bytes of a slice are always the length of the slice.
        unsafe { NbtValueType::read_unaligned(slice, index.cast::<u32>()) as usize }
    }

    #[inline]
    unsafe fn create_reference(_: &[u8], _: Index<Self>) -> &Self {
    unreachable!("Cannot create a reference to a possibly-unaligned slice of `u32`s!")
    }
});
impl_trait!(@slice [u64] => {
    const LENGTH_BYTES: usize = 4;

    #[inline]
    unsafe fn read_length(slice: &[u8], index: Index<Self>) -> usize {
        // SAFETY: The first 4 bytes of a slice are always the length of the slice.
        unsafe { NbtValueType::read_unaligned(slice, index.cast::<u32>()) as usize }
    }

    #[inline]
    unsafe fn create_reference(_: &[u8], _: Index<Self>) -> &Self {
        unreachable!("Cannot create a reference to a possibly-unaligned slice of `u64`s!")
    }
});

// -------------------------------------------------------------------------------------------------

mod sealed {
    use froglight_mutf8::prelude::MStr;

    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}

    impl Sealed for [u8] {}
    impl Sealed for [u32] {}
    impl Sealed for [u64] {}
    impl Sealed for MStr {}
}
