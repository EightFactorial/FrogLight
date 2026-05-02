use core::{
    any::TypeId,
    fmt::{self, Debug},
    hash::Hash,
    marker::PhantomData,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use froglight_mutf8::prelude::*;

use crate::types::borrowed::{Mut, NbtMut, Ref};

/// A reference to a type stored in NBT data.
pub struct NbtItem<'a, T: ?Sized + 'a, Mut: NbtMut> {
    root: Mut::Of<'a, [u8]>,
    index: NbtIndex<T>,
}

impl<'a, T: ?Sized, Mut: NbtMut> NbtItem<'a, T, Mut> {
    /// Create a new [`NbtItem`] with the given root and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for reads and/or writes,
    /// and that it contains a valid `T` at the given index.
    #[inline]
    #[must_use]
    pub const unsafe fn new(root: Mut::Of<'a, [u8]>, index: usize) -> Self {
        Self { root, index: NbtIndex::new(index) }
    }

    /// Create a new [`NbtItem`] with the given root and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for reads and/or writes,
    /// and that it contains a valid `T` at the given index.
    #[inline]
    #[must_use]
    pub const unsafe fn new_indexed(root: Mut::Of<'a, [u8]>, index: NbtIndex<T>) -> Self {
        Self { root, index }
    }
}

impl<'a, T: ?Sized> NbtItem<'a, T, Mut> {
    /// Get this [`Mut`] as a [`Ref`].
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> NbtItem<'_, T, Ref> {
        NbtItem { root: self.root, index: self.index }
    }

    /// Convert this [`Mut`] into a [`Ref`].
    #[inline]
    #[must_use]
    pub const fn into_ref(self) -> NbtItem<'a, T, Ref> {
        NbtItem { root: self.root, index: self.index }
    }
}

impl<T: Copy + Sized, Mut: NbtMut> NbtItem<'_, T, Mut> {
    /// Read the type from the root data slice.
    ///
    /// # Note
    ///
    /// This function does not flip the endianness of the value!
    ///
    /// For integer types you may want to use `to_be` on little-endian
    /// platforms, as NBT data is big-endian.
    #[inline]
    #[must_use]
    pub fn get(&self) -> T { unsafe { self.index.read(&self.root) } }
}

impl<T: Copy + Sized> NbtItem<'_, T, Mut> {
    /// Write the type to the root data slice.
    #[inline]
    pub fn set(&mut self, value: T) { unsafe { self.index.write(value, self.root) } }
}

impl<Mut: NbtMut> NbtItem<'_, [u8], Mut> {
    /// Read the byte slice from the root data slice.
    #[inline]
    #[must_use]
    pub fn get(&self) -> &[u8] { unsafe { self.index.read(&self.root) } }
}

impl<Mut: NbtMut> NbtItem<'_, MStr, Mut> {
    /// Read the string from the root data slice.
    #[inline]
    #[must_use]
    pub fn get(&self) -> &MStr { unsafe { self.index.read(&self.root) } }
}

// -------------------------------------------------------------------------------------------------

/// An index to a type stored in NBT data.
#[repr(transparent)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NbtIndex<T: ?Sized>(usize, PhantomData<T>);

impl<T: ?Sized> NbtIndex<T> {
    /// Create a new [`NbtIndex`] with the given index.
    #[inline]
    #[must_use]
    pub const fn new(index: usize) -> Self { Self(index, PhantomData) }

    /// Get the index of the data in the NBT structure.
    #[inline]
    #[must_use]
    pub const fn index(self) -> usize { self.0 }

    /// Cast this [`NbtIndex`] to a different type.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the data at this index is valid for type
    /// `U`.
    #[inline]
    #[must_use]
    pub const unsafe fn cast<U: ?Sized>(self) -> NbtIndex<U> { NbtIndex(self.0, PhantomData) }
}

impl<T: Copy + Sized> NbtIndex<T> {
    /// Read the type from the given data slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure the slice is valid for reads,
    /// and that it contains a valid `T` at this index.
    #[inline]
    #[must_use]
    #[allow(clippy::missing_panics_doc, reason = "Debug sanity check")]
    pub const unsafe fn read(&self, data: &[u8]) -> T {
        #[cfg(debug_assertions)]
        assert!((self.0 + core::mem::size_of::<T>()) <= data.len(), "Slice is too small to read!");

        // SAFETY: The caller ensures that `data` is valid for reads.
        // SAFETY: The caller ensures that `data` contains a valid `T` at the index.
        unsafe { core::ptr::read_unaligned(data.as_ptr().add(self.0).cast::<T>()) }
    }

    /// Write the type to the given data slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure the slice is valid for writes,
    /// and that it can hold a valid `T` at this index.
    #[inline]
    #[allow(clippy::missing_panics_doc, reason = "Debug sanity check")]
    pub const unsafe fn write(&self, value: T, data: &mut [u8]) {
        #[cfg(debug_assertions)]
        assert!((self.0 + core::mem::size_of::<T>()) <= data.len(), "Slice is too small to write!");

        // SAFETY: The caller ensures that `data` is valid for writes.
        // SAFETY: The caller ensures that `data` can hold a valid `T` at the index.
        unsafe { core::ptr::write_unaligned(data.as_mut_ptr().add(self.0).cast::<T>(), value) }
    }
}

// -------------------------------------------------------------------------------------------------

impl NbtIndex<[u8]> {
    /// Read the length of the byte slice from the given data slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure the slice is valid for reads,
    /// and that it contains a valid byte slice at this index.
    #[inline]
    #[must_use]
    pub const unsafe fn len(self, data: &[u8]) -> u16 {
        // SAFETY: The caller ensures that `data` is valid for reads.
        // SAFETY: The caller ensures that `data` contains a valid byte slice at the
        // index.
        unsafe { NbtIndex::<u16>::new(self.0).read(data) }.to_be()
    }

    /// Read the byte slice from the given data slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure the slice is valid for reads,
    /// and that it contains a valid byte slice at this index.
    #[inline]
    #[must_use]
    #[allow(clippy::missing_panics_doc, reason = "Debug sanity check")]
    pub const unsafe fn read<'data>(&self, data: &'data [u8]) -> &'data [u8] {
        #[cfg(debug_assertions)]
        {
            // SAFETY: The caller ensures that `data` is valid for reads.
            // SAFETY: The caller ensures that `data` contains a valid byte slice at the
            // index.
            let len = unsafe { self.len(data) } as usize;
            assert!((self.0 + 2 + len) <= data.len(), "Slice is too small to read the slice!");
        }

        // SAFETY: The caller ensures that `data` is valid for reads.
        // SAFETY: The caller ensures that `data` contains a valid byte slice at the
        // index.
        unsafe {
            let len = self.len(data) as usize;
            let ptr = data.as_ptr().add(self.0 + 2);
            core::slice::from_raw_parts(ptr, len)
        }
    }
}

impl NbtIndex<MStr> {
    /// Read the length of the string from the given data slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure the slice is valid for reads,
    /// and that it contains a valid [`MStr`] at this index.
    #[inline]
    #[must_use]
    pub const unsafe fn len(self, data: &[u8]) -> u16 {
        // SAFETY: The caller ensures that `data` is valid for reads.
        // SAFETY: The caller ensures that `data` contains a valid `MStr` at the index.
        unsafe { NbtIndex::<u16>::new(self.0).read(data) }.to_be()
    }

    /// Read the string from the given data slice.
    ///
    /// # Note
    ///
    /// This function does not perform any MUTF-8 validation!
    ///
    /// # Safety
    ///
    /// The caller must ensure the slice is valid for reads,
    /// and that it contains a valid [`MStr`] at this index.
    #[inline]
    #[must_use]
    #[allow(clippy::missing_panics_doc, reason = "Debug sanity check")]
    pub const unsafe fn read<'data>(&self, data: &'data [u8]) -> &'data MStr {
        #[cfg(debug_assertions)]
        {
            // SAFETY: The caller ensures that `data` is valid for reads.
            // SAFETY: The caller ensures that `data` contains a valid `MStr` at the index.
            let len = unsafe { self.len(data) } as usize;
            assert!((self.0 + 2 + len) <= data.len(), "Slice is too small to read the string!");
        }

        // SAFETY: The caller ensures that `data` is valid for reads.
        // SAFETY: The caller ensures that `data` contains a valid `MStr` at the index.
        unsafe {
            let len = self.len(data) as usize;
            let ptr = data.as_ptr().add(self.0 + 2);
            MStr::from_mutf8_unchecked(core::slice::from_raw_parts(ptr, len))
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: ?Sized> Debug for NbtIndex<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NbtRef").field(&self.0).finish()
    }
}

impl<T: ?Sized> Copy for NbtIndex<T> {}
impl<T: ?Sized> Clone for NbtIndex<T> {
    #[inline]
    fn clone(&self) -> Self { *self }
}

impl<T: ?Sized> Eq for NbtIndex<T> {}
impl<T1: ?Sized, T2: ?Sized> PartialEq<NbtIndex<T2>> for NbtIndex<T1> {
    #[inline]
    fn eq(&self, other: &NbtIndex<T2>) -> bool { self.0 == other.0 }
}

impl<T: ?Sized + 'static> Hash for NbtIndex<T> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        TypeId::of::<T>().hash(state);
        self.0.hash(state);
    }
}

impl<T: ?Sized> Add<usize> for NbtIndex<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output { Self::new(self.0 + rhs) }
}
impl<T: ?Sized> AddAssign<usize> for NbtIndex<T> {
    #[inline]
    fn add_assign(&mut self, rhs: usize) { self.0 += rhs; }
}

impl<T: ?Sized> Sub<usize> for NbtIndex<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: usize) -> Self::Output { Self::new(self.0 - rhs) }
}
impl<T: ?Sized> SubAssign<usize> for NbtIndex<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) { self.0 -= rhs; }
}
