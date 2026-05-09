//! TODO

use froglight_mutf8::prelude::MStr;

use crate::types::borrowed::{
    core::{IndexCore, Mut, NbtAccess, Ref},
    index::{Index, IndexedListType, IndexedMapType},
    reference::{IndexedReference, NbtSliceType, NbtValueType},
};

cfg_select! {
    feature = "alloc" => {
        /// A typed NBT List that is indexed by an [`IndexCore`].
        pub struct IndexedList<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'data = super::core::BorrowedCore<'data, A>> {
            core: A::CORE<'data, C>,
            index: Index<T>,
        }

        /// An NBT List that is indexed by an [`IndexCore`].
        pub enum IndexedValueList<'data, A: NbtAccess, C: IndexCore<A> + 'data = super::core::BorrowedCore<'data, A>> {
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
    }
    _ => {
        /// A typed NBT List that is indexed by an [`IndexCore`].
        pub struct IndexedList<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'data> {
            core: A::CORE<'data, C>,
            index: Index<T>,
        }

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
            ByteArray(IndexedList<'data, u8, A, C>),
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
    }
}

impl<'data, T: ?Sized, A: NbtAccess, C: IndexCore<A> + 'data> IndexedList<'data, T, A, C> {
    /// Create a new [`IndexedList`] from the given core and [`Index`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given core.
    #[inline]
    #[must_use]
    pub const unsafe fn new(core: A::CORE<'data, C>, index: Index<T>) -> Self {
        Self { core, index }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data, T: NbtValueType, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data>
    IndexedList<'data, T, A, C>
{
    /// Get the length of the list.
    #[must_use]
    pub fn len(&self) -> usize {
        // SAFETY: All `NbtValueType`-type lists start with a 4-byte `u32` length
        // prefix, followed by the list elements.
        unsafe {
            IndexedReference::<u32, Ref>::new(
                <C as IndexCore<A>>::root(&self.core),
                self.index.cast::<u32>(),
            )
            .get() as usize
        }
    }

    /// Return `true` if the length of the list is zero.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get a reference to the element at the given index, if it is present,
    /// else `None`.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<IndexedReference<'_, T, Ref>> {
        if index < self.len() {
            // SAFETY: A bounds check was just performed.
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    /// Get a reference to the element at the given index without performing
    /// bounds checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is within the bounds of the list.
    #[must_use]
    pub unsafe fn get_unchecked(&self, index: usize) -> IndexedReference<'_, T, Ref> {
        // SAFETY: All `NbtValueType`-type lists start with a 4-byte `u32` length
        // prefix, followed by the list elements.
        unsafe {
            let offset = core::mem::size_of::<u32>() + (index * core::mem::size_of::<T>());
            let index = Index::<T>::new(self.index.value() + offset);
            IndexedReference::<T, Ref>::new(<C as IndexCore<A>>::root(&self.core), index)
        }
    }
}

impl<'data, T: NbtValueType, C: IndexCore<Mut> + 'data> IndexedList<'data, T, Mut, C> {
    /// Get a mutable reference to the element at the given index, if it is
    /// present, else `None`.
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<IndexedReference<'_, T, Mut>> {
        if index < self.len() {
            // SAFETY: A bounds check was just performed.
            Some(unsafe { self.get_mut_unchecked(index) })
        } else {
            None
        }
    }

    /// Get a mutable reference to the element at the given index without
    /// performing bounds checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is within the bounds of the list.
    #[must_use]
    pub unsafe fn get_mut_unchecked(&mut self, index: usize) -> IndexedReference<'_, T, Mut> {
        // SAFETY: All `NbtValueType` lists start with a 4-byte `u32` length prefix,
        // followed by the list elements.
        unsafe {
            let offset = core::mem::size_of::<u32>() + (index * core::mem::size_of::<T>());
            let index = Index::<T>::new(self.index.value() + offset);
            IndexedReference::<T, Mut>::new(self.core.root_mut(), index)
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data, T: NbtSliceType + ?Sized, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data>
    IndexedList<'data, T, A, C>
{
    /// Get the number of slices in the list.
    #[must_use]
    pub fn slice_count(&self) -> usize {
        unsafe { <C as IndexCore<A>>::entry_range(&self.core, self.index.value()).len() }
    }

    /// Get a reference to the element at the given index, if it is present,
    /// else `None`.
    #[must_use]
    pub fn get_slice(&self, index: usize) -> Option<IndexedReference<'_, T, Ref>> {
        unsafe {
            // SAFETY: `self.index.value()` is the range index for `NbtSliceType` types.
            let entry =
                <C as IndexCore<A>>::entry_range(&self.core, self.index.value()).get(index)?;

            // SAFETY: `T` is guaranteed to be the type of value in the list.
            Some(IndexedReference::<T, Ref>::new(
                <C as IndexCore<A>>::root(&self.core),
                Index::new(entry.value().index()),
            ))
        }
    }

    /// Get a reference to the element at the given index without performing
    /// bounds checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is within the bounds of the list.
    #[must_use]
    pub unsafe fn get_slice_unchecked(&self, index: usize) -> IndexedReference<'_, T, Ref> {
        unsafe {
            // SAFETY: `self.index.value()` is the range index for `NbtSliceType` types.
            // SAFETY: The caller guarantees that `index` is within the bounds of the list.
            let entry = <C as IndexCore<A>>::entry_range(&self.core, self.index.value())
                .get_unchecked(index);

            // SAFETY: `T` is guaranteed to be the type of value in the list.
            IndexedReference::<T, Ref>::new(
                <C as IndexCore<A>>::root(&self.core),
                Index::new(entry.value().index()),
            )
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data>
    IndexedList<'data, IndexedListType, A, C>
{
    /// Get the number of lists in the list.
    #[must_use]
    pub fn list_count(&self) -> usize {
        unsafe { <C as IndexCore<A>>::entry_range(&self.core, self.index.value()).len() }
    }

    /// Get a reference to the element at the given index, if it is present,
    /// else `None`.
    #[must_use]
    pub fn get_list(&self, index: usize) -> Option<IndexedReference<'_, IndexedListType, Ref>> {
        unsafe {
            // SAFETY: `self.index.value()` is the range index for `IndexedListType` lists.
            let entry =
                <C as IndexCore<A>>::entry_range(&self.core, self.index.value()).get(index)?;

            // SAFETY: `IndexedListType` is guaranteed to be the type of value in the list.
            Some(IndexedReference::<IndexedListType, Ref>::new(
                <C as IndexCore<A>>::root(&self.core),
                Index::new(entry.value().index()),
            ))
        }
    }

    /// Get a reference to the element at the given index without performing
    /// bounds checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is within the bounds of the list.
    #[must_use]
    pub unsafe fn get_list_unchecked(
        &self,
        index: usize,
    ) -> IndexedReference<'_, IndexedListType, Ref> {
        unsafe {
            // SAFETY: `self.index.value()` is the range index for `IndexedListType` lists.
            // SAFETY: The caller guarantees that `index` is within the bounds of the list.
            let entry = <C as IndexCore<A>>::entry_range(&self.core, self.index.value())
                .get_unchecked(index);

            // SAFETY: `IndexedListType` is guaranteed to be the type of value in the list.
            IndexedReference::<IndexedListType, Ref>::new(
                <C as IndexCore<A>>::root(&self.core),
                Index::new(entry.value().index()),
            )
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data>
    IndexedList<'data, IndexedMapType, A, C>
{
    /// Get the number of compounds in the list.
    #[must_use]
    pub fn map_count(&self) -> usize {
        unsafe { <C as IndexCore<A>>::entry_range(&self.core, self.index.value()).len() }
    }

    /// Get a reference to the element at the given index, if it is present,
    /// else `None`.
    #[must_use]
    pub fn get_map(&self, index: usize) -> Option<IndexedReference<'_, IndexedMapType, Ref>> {
        unsafe {
            // SAFETY: `self.index.value()` is the range index for `IndexedMapType` lists.
            let entry =
                <C as IndexCore<A>>::entry_range(&self.core, self.index.value()).get(index)?;

            // SAFETY: `IndexedMapType` is guaranteed to be the type of value in the list.
            Some(IndexedReference::<IndexedMapType, Ref>::new(
                <C as IndexCore<A>>::root(&self.core),
                Index::new(entry.value().index()),
            ))
        }
    }

    /// Get a reference to the element at the given index without performing
    /// bounds checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is within the bounds of the list.
    #[must_use]
    pub unsafe fn get_map_unchecked(
        &self,
        index: usize,
    ) -> IndexedReference<'_, IndexedMapType, Ref> {
        unsafe {
            // SAFETY: `self.index.value()` is the range index for `IndexedMapType` lists.
            // SAFETY: The caller guarantees that `index` is within the bounds of the list.
            let entry = <C as IndexCore<A>>::entry_range(&self.core, self.index.value())
                .get_unchecked(index);

            // SAFETY: `IndexedMapType` is guaranteed to be the type of value in the list.
            IndexedReference::<IndexedMapType, Ref>::new(
                <C as IndexCore<A>>::root(&self.core),
                Index::new(entry.value().index()),
            )
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<'data, A: NbtAccess, C: IndexCore<Ref> + IndexCore<A> + 'data> IndexedValueList<'data, A, C> {
    /// Get the length of the list.
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
            Self::ByteArray(list) => list.slice_count(),
            Self::String(list) => list.slice_count(),
            Self::List(list) => list.list_count(),
            Self::Compound(list) => list.map_count(),
            Self::IntArray(list) => list.slice_count(),
            Self::LongArray(list) => list.slice_count(),
        }
    }

    /// Return `true` if the length of the list is zero.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Get a reference to a [`u8`] list, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::Byte`] list.
    #[inline]
    #[must_use]
    pub const fn as_byte(&self) -> Option<&IndexedList<'data, u8, A, C>> {
        if let Self::Byte(list) = self { Some(list) } else { None }
    }

    /// Get a reference to a [`u16`] list, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::Short`] list.
    #[inline]
    #[must_use]
    pub const fn as_short(&self) -> Option<&IndexedList<'data, u16, A, C>> {
        if let Self::Short(list) = self { Some(list) } else { None }
    }

    /// Get a reference to a [`u32`] list, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::Int`] list.
    #[inline]
    #[must_use]
    pub const fn as_int(&self) -> Option<&IndexedList<'data, u32, A, C>> {
        if let Self::Int(list) = self { Some(list) } else { None }
    }

    /// Get a reference to a [`u64`] list, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::Long`] list.
    #[inline]
    #[must_use]
    pub const fn as_long(&self) -> Option<&IndexedList<'data, u64, A, C>> {
        if let Self::Long(list) = self { Some(list) } else { None }
    }

    /// Get a reference to a [`f32`] list, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::Float`] list.
    #[inline]
    #[must_use]
    pub const fn as_float(&self) -> Option<&IndexedList<'data, f32, A, C>> {
        if let Self::Float(list) = self { Some(list) } else { None }
    }

    /// Get a reference to a [`f64`] list, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::Double`] list.
    #[inline]
    #[must_use]
    pub const fn as_double(&self) -> Option<&IndexedList<'data, f64, A, C>> {
        if let Self::Double(list) = self { Some(list) } else { None }
    }

    /// Get a reference to a [`u8`] array list, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::ByteArray`]
    /// list.
    #[inline]
    #[must_use]
    pub const fn as_byte_array(&self) -> Option<&IndexedList<'data, [u8], A, C>> {
        if let Self::ByteArray(list) = self { Some(list) } else { None }
    }

    /// Get a reference to an [`MStr`] list, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::String`] list.
    #[inline]
    #[must_use]
    pub const fn as_string(&self) -> Option<&IndexedList<'data, MStr, A, C>> {
        if let Self::String(list) = self { Some(list) } else { None }
    }

    /// Get a reference to a list of lists, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::List`] list.
    #[inline]
    #[must_use]
    pub const fn as_list(&self) -> Option<&IndexedList<'data, IndexedListType, A, C>> {
        if let Self::List(list) = self { Some(list) } else { None }
    }

    /// Get a reference to a list of compounds, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::Compound`] list.
    #[inline]
    #[must_use]
    pub const fn as_compound(&self) -> Option<&IndexedList<'data, IndexedMapType, A, C>> {
        if let Self::Compound(list) = self { Some(list) } else { None }
    }

    /// Get a reference to a [`u32`] array list, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::IntArray`] list.
    #[inline]
    #[must_use]
    pub const fn as_int_array(&self) -> Option<&IndexedList<'data, [u32], A, C>> {
        if let Self::IntArray(list) = self { Some(list) } else { None }
    }

    /// Get a reference to a [`u64`] array list, if it is one.
    ///
    /// Returns `None` if the list is not a [`IndexedValueList::LongArray`]
    /// list.
    #[inline]
    #[must_use]
    pub const fn as_long_array(&self) -> Option<&IndexedList<'data, [u64], A, C>> {
        if let Self::LongArray(list) = self { Some(list) } else { None }
    }
}
