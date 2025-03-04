use std::marker::PhantomData;

use super::{NbtCompoundRef, NbtListTagRef};
use crate::mutf8::Mutf8Str;

/// A length-prefixed array of items.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PrefixedArray<'a, T: PrefixedArrayItem<'a>>(usize, &'a [u8], PhantomData<T>);

impl<'a, T: PrefixedArrayItem<'a>> PrefixedArray<'a, T> {
    /// Return the next item in the array.
    #[must_use]
    pub fn next_item(&mut self) -> Option<T> {
        if self.0 > 0 {
            let (size, data) = T::size_of(self.1);
            let (item, data) = data.split_at(size);

            // Update the remaining data
            self.0 -= 1;
            self.1 = data;

            Some(unsafe { T::from_bytes(item) })
        } else {
            None
        }
    }

    /// Return the number of items in the array.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.0 }

    /// Returns `true` if the array is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0 == 0 }

    /// Create a [`Vec`] from the items in the array.
    #[inline]
    #[must_use]
    pub fn to_vec(self) -> Vec<T> { self.into() }

    /// Create a new [`PrefixedArray`] from the given data.
    ///
    /// # Safety
    /// The caller must ensure that the data is a valid
    /// length-prefixed array of the given type.
    #[inline]
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub const unsafe fn from_bytes(data: &'a [u8]) -> Self {
        let (&length, data) = data.split_first_chunk::<4>().unwrap();
        Self(u32::from_be_bytes(length) as usize, data, PhantomData)
    }
}

impl<'a, T: PrefixedArrayItem<'a>> From<PrefixedArray<'a, T>> for Vec<T> {
    fn from(array: PrefixedArray<'a, T>) -> Self { array.into_iter().collect() }
}

// -------------------------------------------------------------------------------------------------

/// A type that can be read from a [`PrefixedArray`].
pub trait PrefixedArrayItem<'a> {
    /// Create a new instance of the type from the given bytes.
    ///
    /// # Safety
    /// The caller must ensure that the data is a valid
    /// representation of the type.
    unsafe fn from_bytes(data: &'a [u8]) -> Self;
    /// Return the size of the item in bytes and the remaining data
    /// after reading the size.
    fn size_of(data: &'a [u8]) -> (usize, &'a [u8]);
}

impl PrefixedArrayItem<'_> for i8 {
    #[expect(clippy::cast_possible_wrap)]
    unsafe fn from_bytes(data: &[u8]) -> Self { data[0] as i8 }

    fn size_of(data: &[u8]) -> (usize, &[u8]) { (1, data) }
}
impl PrefixedArrayItem<'_> for i16 {
    unsafe fn from_bytes(data: &[u8]) -> Self { i16::from_be_bytes(data.try_into().unwrap()) }

    fn size_of(data: &[u8]) -> (usize, &[u8]) { (2, data) }
}
impl PrefixedArrayItem<'_> for i32 {
    unsafe fn from_bytes(data: &[u8]) -> Self { i32::from_be_bytes(data.try_into().unwrap()) }

    fn size_of(data: &[u8]) -> (usize, &[u8]) { (4, data) }
}
impl PrefixedArrayItem<'_> for i64 {
    unsafe fn from_bytes(data: &[u8]) -> Self { i64::from_be_bytes(data.try_into().unwrap()) }

    fn size_of(data: &[u8]) -> (usize, &[u8]) { (8, data) }
}

impl PrefixedArrayItem<'_> for f32 {
    unsafe fn from_bytes(data: &[u8]) -> Self { f32::from_be_bytes(data.try_into().unwrap()) }

    fn size_of(data: &[u8]) -> (usize, &[u8]) { (4, data) }
}
impl PrefixedArrayItem<'_> for f64 {
    unsafe fn from_bytes(data: &[u8]) -> Self { f64::from_be_bytes(data.try_into().unwrap()) }

    fn size_of(data: &[u8]) -> (usize, &[u8]) { (8, data) }
}

impl<'a> PrefixedArrayItem<'a> for &'a Mutf8Str {
    unsafe fn from_bytes(data: &'a [u8]) -> Self { Mutf8Str::from_bytes(data) }

    fn size_of(data: &[u8]) -> (usize, &[u8]) {
        let (&length, data) = data.split_first_chunk::<2>().unwrap();
        let length = u16::from_be_bytes(length) as usize;
        (length, data)
    }
}

impl<'a> PrefixedArrayItem<'a> for NbtCompoundRef<'a> {
    unsafe fn from_bytes(data: &'a [u8]) -> Self {
        // SAFETY: The caller ensured the data is valid NBT.
        unsafe { NbtCompoundRef::from_bytes(data) }
    }

    fn size_of(data: &'a [u8]) -> (usize, &'a [u8]) {
        let size = NbtCompoundRef::size_of(data).expect("Invalid NBTCompoundRef");
        (size, data)
    }
}
impl<'a> PrefixedArrayItem<'a> for NbtListTagRef<'a> {
    unsafe fn from_bytes(data: &'a [u8]) -> Self {
        // SAFETY: The caller ensured the data is valid NBT.
        unsafe { NbtListTagRef::from_bytes(data) }
    }

    fn size_of(data: &'a [u8]) -> (usize, &'a [u8]) {
        let size = NbtListTagRef::size_of(data).expect("Invalid NBTListTagRef");
        (size, data)
    }
}

impl<'a, T: PrefixedArrayItem<'a>> PrefixedArrayItem<'a> for PrefixedArray<'a, T> {
    unsafe fn from_bytes(data: &'a [u8]) -> Self {
        // SAFETY: The caller ensured the data is valid PrefixedArray.
        unsafe { PrefixedArray::from_bytes(data) }
    }

    fn size_of(data: &'a [u8]) -> (usize, &'a [u8]) {
        let (&item_count, data) = data.split_first_chunk::<4>().unwrap();
        let item_count = u32::from_be_bytes(item_count) as usize;

        let mut split_data = data;
        let size = (0..item_count).fold(4, |size, _| {
            let (item_size, data) = T::size_of(split_data);
            split_data = &data[item_size..];
            size + item_size
        });

        (size, data)
    }
}

// -------------------------------------------------------------------------------------------------

/// An iterator over the items in a [`PrefixedArray`].
pub struct PrefixedArrayIter<'a, T: PrefixedArrayItem<'a>>(PrefixedArray<'a, T>);

impl<'a, T: PrefixedArrayItem<'a>> Iterator for PrefixedArrayIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> { self.0.next_item() }
}

impl<'a, T: PrefixedArrayItem<'a>> IntoIterator for PrefixedArray<'a, T> {
    type IntoIter = PrefixedArrayIter<'a, T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter { PrefixedArrayIter(self) }
}
