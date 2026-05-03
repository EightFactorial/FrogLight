//! TODO

use core::{
    any::{Any, TypeId},
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use froglight_mutf8::prelude::MStr;

/// A reference to a type `T` in a byte slice.
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BorrowedRef<'data, T: ?Sized> {
    root: &'data [u8],
    index: BorrowedIndex<T>,
}

impl<'data, T: ?Sized> BorrowedRef<'data, T> {
    /// Create a new [`BorrowedRef`] using the given data and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for reads,
    /// and that it contains a valid `T` at the position specified by `index`.
    #[inline]
    #[must_use]
    pub const unsafe fn new(root: &'data [u8], index: BorrowedIndex<T>) -> Self {
        Self { root, index }
    }

    /// Cast this [`BorrowedRef`] to a different type.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` contains a valid `U` at the index's
    /// position.
    #[inline]
    #[must_use]
    pub const unsafe fn cast<U: ?Sized>(self) -> BorrowedRef<'data, U> {
        BorrowedRef { root: self.root, index: unsafe { self.index.cast() } }
    }
}

impl<T: BorrowedPOD> BorrowedRef<'_, T> {
    /// Get the value of this [`BorrowedRef`].
    #[inline]
    #[must_use]
    pub fn get_value(&self) -> T {
        // SAFETY: `BorrowedRef` guarantees this is valid
        unsafe { T::read_unaligned(self.root, self.index) }
    }
}

impl<'data, T: BorrowedSlice + ?Sized> BorrowedRef<'data, T> {
    /// Get a reference to the [`BorrowedRef`]'s slice.
    #[inline]
    #[must_use]
    pub fn get_ref(&self) -> &'data T {
        // SAFETY: `BorrowedRef` guarantees this is valid
        unsafe { T::get_slice_ref(self.root, self.index) }
    }

    /// Get the length of this [`BorrowedRef`]'s slice.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        // SAFETY: `BorrowedRef` guarantees this is valid
        unsafe { T::read_length(self.root, self.index) }
    }

    /// Returns `true` if this [`BorrowedRef`]'s slice is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

// -------------------------------------------------------------------------------------------------

/// A mutable reference to a type `T` in a byte slice.
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BorrowedMut<'data, T: ?Sized> {
    root: &'data mut [u8],
    index: BorrowedIndex<T>,
}

impl<'data, T: ?Sized> BorrowedMut<'data, T> {
    /// Create a new [`BorrowedRef`] using the given data and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for reads and writes,
    /// and that it contains a valid `T` at the position specified by `index`.
    #[inline]
    #[must_use]
    pub const unsafe fn new(root: &'data mut [u8], index: BorrowedIndex<T>) -> Self {
        Self { root, index }
    }

    /// Get this [`BorrowedMut`] as a [`BorrowedRef`].
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> BorrowedRef<'_, T> {
        BorrowedRef { root: self.root, index: self.index }
    }

    /// Convert this [`BorrowedMut`] into a [`BorrowedRef`].
    #[inline]
    #[must_use]
    pub const fn into_ref(self) -> BorrowedRef<'data, T> {
        BorrowedRef { root: self.root, index: self.index }
    }

    /// Cast this [`BorrowedMut`] to a different type.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` contains a valid `U` at the index's
    /// position.
    #[inline]
    #[must_use]
    pub const unsafe fn cast<U: ?Sized>(self) -> BorrowedMut<'data, U> {
        BorrowedMut { root: self.root, index: unsafe { self.index.cast() } }
    }
}

impl<T: BorrowedPOD> BorrowedMut<'_, T> {
    /// Get the value of this [`BorrowedMut`].
    #[inline]
    #[must_use]
    pub fn get_value(&self) -> T {
        // SAFETY: `BorrowedMut` guarantees this is valid
        unsafe { T::read_unaligned(self.root, self.index) }
    }

    /// Set the value of this [`BorrowedMut`].
    #[inline]
    pub fn set_value(&mut self, value: T) {
        // SAFETY: `BorrowedMut` guarantees this is valid
        unsafe { T::write_unaligned(self.root, value, self.index) }
    }
}

impl<T: BorrowedSlice> BorrowedMut<'_, T> {
    /// Get a reference to the [`BorrowedMut`]'s slice.
    #[inline]
    #[must_use]
    pub fn get_ref(&self) -> &T {
        // SAFETY: `BorrowedMut` guarantees this is valid
        unsafe { T::get_slice_ref(self.root, self.index) }
    }

    /// Get a mutable reference to the [`BorrowedMut`]'s slice.
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self) -> &mut T {
        // SAFETY: `BorrowedMut` guarantees this is valid
        unsafe { T::get_slice_mut(self.root, self.index) }
    }

    /// Get the length of this [`BorrowedMut`]'s slice.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        // SAFETY: `BorrowedMut` guarantees this is valid
        unsafe { T::read_length(self.root, self.index) }
    }

    /// Returns `true` if this [`BorrowedMut`]'s slice is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

// -------------------------------------------------------------------------------------------------

/// A trait for POD types that can be used with [`BorrowedRef`] and
/// [`BorrowedMut`].
pub trait BorrowedPOD: Any + Copy + sealed::Sealed {
    /// Read a value of this type from the given data and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for reads,
    /// and that it contains a valid value of this type at given index.
    unsafe fn read_unaligned(root: &[u8], index: BorrowedIndex<Self>) -> Self;

    /// Write a value of this type to the given data and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for writes,
    /// and that it can hold a valid value of this type at given index.
    unsafe fn write_unaligned(root: &mut [u8], value: Self, index: BorrowedIndex<Self>);
}

/// A trait for slice-like types that can be used with [`BorrowedRef`] and
/// [`BorrowedMut`].
pub trait BorrowedSlice: sealed::Sealed {
    /// Read the length of this slice from the given data and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for reads,
    /// and that it contains a valid slice of this type at given index.
    unsafe fn read_length(root: &[u8], index: BorrowedIndex<Self>) -> usize;

    /// Get a reference to this slice from the given data and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for reads,
    /// and that it contains a valid slice of this type at given index.
    unsafe fn get_slice_ref(root: &[u8], index: BorrowedIndex<Self>) -> &Self;

    /// Get a mutable reference to this slice from the given data and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `root` is valid for reads and writes,
    /// and that it contains a valid slice of this type at given index.
    unsafe fn get_slice_mut(root: &mut [u8], index: BorrowedIndex<Self>) -> &mut Self;
}

macro_rules! impl_trait {
    (@integer $($ty:ty),*) => {
        $(
            impl BorrowedPOD for $ty {
                #[inline]
                unsafe fn read_unaligned(root: &[u8], index: BorrowedIndex<Self>) -> Self {
                    // SAFETY: The caller ensures that `read_unaligned` is valid.
                    unsafe { core::ptr::read_unaligned(root.as_ptr().add(index.index()).cast::<Self>()).to_be() }
                }

                #[inline]
                unsafe fn write_unaligned(root: &mut [u8], value: Self, index: BorrowedIndex<Self>) {
                    // SAFETY: The caller ensures that `read_unaligned` is valid.
                    unsafe { core::ptr::write_unaligned(root.as_mut_ptr().add(index.index()).cast::<Self>(), value.to_be()); }
                }
            }

            impl sealed::Sealed for $ty {}
        )*
    };
    (@float $($ty:ty),*) => {
        $(
            impl BorrowedPOD for $ty {
                #[inline]
                unsafe fn read_unaligned(root: &[u8], index: BorrowedIndex<Self>) -> Self {
                    // SAFETY: The caller ensures that `read_unaligned` is valid.
                    unsafe { core::ptr::read_unaligned(root.as_ptr().add(index.index()).cast::<Self>()) }
                }

                #[inline]
                unsafe fn write_unaligned(root: &mut [u8], value: Self, index: BorrowedIndex<Self>) {
                    // SAFETY: The caller ensures that `read_unaligned` is valid.
                    unsafe { core::ptr::write_unaligned(root.as_mut_ptr().add(index.index()).cast::<Self>(), value); }
                }
            }

            impl sealed::Sealed for $ty {}
        )*
    };
}

impl_trait!(@integer i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
impl_trait!(@float f32, f64);

impl BorrowedSlice for [u8] {
    #[inline]
    unsafe fn read_length(root: &[u8], index: BorrowedIndex<Self>) -> usize {
        // SAFETY: The caller ensures that `read_length` is valid.
        unsafe {
            #[expect(clippy::cast_ptr_alignment, reason = "Using with `read_unaligned`")]
            let ptr = root.as_ptr().add(index.index()).cast::<u16>();
            usize::from(core::ptr::read_unaligned(ptr).to_be())
        }
    }

    #[inline]
    unsafe fn get_slice_ref(root: &[u8], index: BorrowedIndex<Self>) -> &Self {
        // SAFETY: The caller ensures that `get_slice_ref` is valid.
        unsafe {
            let length = Self::read_length(root, index);
            let ptr = root.as_ptr().add(index.index() + 2);
            core::slice::from_raw_parts(ptr, length)
        }
    }

    #[inline]
    unsafe fn get_slice_mut(root: &mut [u8], index: BorrowedIndex<Self>) -> &mut Self {
        // SAFETY: The caller ensures that `get_slice_mut` is valid.
        unsafe {
            let length = Self::read_length(root, index);
            let ptr = root.as_mut_ptr().add(index.index() + 2);
            core::slice::from_raw_parts_mut(ptr, length)
        }
    }
}

impl BorrowedSlice for MStr {
    #[inline]
    unsafe fn read_length(root: &[u8], index: BorrowedIndex<Self>) -> usize {
        // SAFETY: The caller ensures that `read_length` is valid.
        // SAFETY: The layouts of `MStr` and `[u8]` are the same.
        unsafe { <[u8] as BorrowedSlice>::read_length(root, index.cast()) }
    }

    #[inline]
    unsafe fn get_slice_ref(root: &[u8], index: BorrowedIndex<Self>) -> &Self {
        // SAFETY: The caller ensures that `get_slice_ref` is valid.
        // SAFETY: The layouts of `MStr` and `[u8]` are the same.
        unsafe {
            let slice = <[u8] as BorrowedSlice>::get_slice_ref(root, index.cast());
            MStr::from_mutf8_unchecked(slice)
        }
    }

    #[inline]
    unsafe fn get_slice_mut(root: &mut [u8], index: BorrowedIndex<Self>) -> &mut Self {
        // SAFETY: The caller ensures that `get_slice_mut` is valid.
        // SAFETY: The layouts of `MStr` and `[u8]` are the same.
        unsafe {
            let slice = <[u8] as BorrowedSlice>::get_slice_mut(root, index.cast());
            MStr::from_mutf8_mut_unchecked(slice)
        }
    }
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for [u8] {}
    impl Sealed for froglight_mutf8::prelude::MStr {}
}

// -------------------------------------------------------------------------------------------------

/// A byte-index to a type `T`.
#[repr(transparent)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BorrowedIndex<T: ?Sized>(usize, PhantomData<T>);

impl<T: ?Sized> BorrowedIndex<T> {
    /// Create a new [`BorrowedIndex`] using the given index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the type `T` in
    /// whatever context it is used.
    #[inline]
    #[must_use]
    pub const unsafe fn new(index: usize) -> Self { Self(index, PhantomData) }

    /// Cast this [`BorrowedIndex`] to a different type.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the new type `U` in
    /// whatever context it is used.
    #[inline]
    #[must_use]
    pub const unsafe fn cast<U: ?Sized>(self) -> BorrowedIndex<U> {
        BorrowedIndex(self.0, PhantomData)
    }

    /// Get the index contained in this [`BorrowedIndex`].
    #[inline]
    #[must_use]
    pub const fn index(&self) -> usize { self.0 }
}

impl<T: ?Sized> fmt::Debug for BorrowedIndex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BorrowedIndex").field(&self.0).finish()
    }
}
impl<T: ?Sized> fmt::Display for BorrowedIndex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

impl<T: ?Sized> Copy for BorrowedIndex<T> {}
impl<T: ?Sized> Clone for BorrowedIndex<T> {
    #[inline]
    fn clone(&self) -> Self { *self }
}

impl<T: ?Sized> Eq for BorrowedIndex<T> {}
impl<T: ?Sized> PartialEq for BorrowedIndex<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl<T: ?Sized> Ord for BorrowedIndex<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering { self.0.cmp(&other.0) }
}
impl<T: ?Sized> PartialOrd for BorrowedIndex<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl<T: Any> Hash for BorrowedIndex<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        TypeId::of::<T>().hash(state);
        self.0.hash(state);
    }
}

impl<T: ?Sized> Add<usize> for BorrowedIndex<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output { BorrowedIndex(self.0 + rhs, PhantomData) }
}
impl<T: ?Sized> AddAssign<usize> for BorrowedIndex<T> {
    #[inline]
    fn add_assign(&mut self, rhs: usize) { self.0 += rhs; }
}

impl<T: ?Sized> Sub<usize> for BorrowedIndex<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: usize) -> Self::Output { BorrowedIndex(self.0 - rhs, PhantomData) }
}
impl<T: ?Sized> SubAssign<usize> for BorrowedIndex<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) { self.0 -= rhs; }
}
