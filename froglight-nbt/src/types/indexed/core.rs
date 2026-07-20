//! TODO

use core::{fmt::Debug, hash::Hash, ops::Deref};

#[cfg(feature = "froglight-facet")]
use facet::Partial;
#[cfg(feature = "froglight-facet")]
use froglight_facet::facet::{WithFnAttr, prelude::*};

use crate::types::indexed::index::EntryIndex;

/// A trait for an index of NBT entries.
pub trait IndexCore<A: NbtAccess>: Eq {
    /// Get a reference to the root NBT data slice.
    #[must_use]
    fn root(&self) -> &[u8];

    /// Get a reference to the [`EntryIndex`]es of this NBT structure.
    #[must_use]
    fn entries(&self) -> &[EntryIndex];

    /// Get the [`EntryIndex`]es for the given compound/list index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given index is valid.
    #[must_use]
    unsafe fn entry_range(&self, index: usize) -> &[EntryIndex];

    /// Get a mutable reference to the root NBT data slice.
    #[must_use]
    fn root_mut(&mut self) -> &mut [u8]
    where
        A: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>;

    /// Get a mutable reference to the [`EntryIndex`]es of this NBT structure.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the list of entries is still valid
    /// if it is modified.
    #[must_use]
    unsafe fn entries_mut(&mut self) -> &mut [EntryIndex]
    where
        A: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>;

    /// Get a mutable reference to the [`EntryIndex`]es for the given
    /// compound/list index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given index is valid and that the list
    /// of entries is still valid if it is modified.
    #[must_use]
    unsafe fn entry_range_mut(&mut self, index: usize) -> &mut [EntryIndex]
    where
        A: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>;

    /// The [`WithFnAttr`] for this named NBT using this [`IndexCore`].
    #[cfg(feature = "froglight-facet")]
    const WITH_NAMED: WithFnAttr = WithFnAttr::using(
        Self::serialize_named,
        Self::deserialize_named::<false>,
        Self::deserialize_named::<true>,
    );

    /// The [`WithFnAttr`] for this unnamed NBT using this [`IndexCore`].
    #[cfg(feature = "froglight-facet")]
    const WITH_UNNAMED: WithFnAttr = WithFnAttr::using(
        Self::serialize_unnamed,
        Self::deserialize_unnamed::<false>,
        Self::deserialize_unnamed::<true>,
    );

    /// The [`WithFnAttr`] for optional unnamed NBT using this [`IndexCore`].
    #[cfg(feature = "froglight-facet")]
    const WITH_OPT_UNNAMED: WithFnAttr = WithFnAttr::using(
        |item, writer| {
            let option = item.peek().into_option()?;
            if let Some(inner) = option.value() {
                let item = SerializeItem::new(inner, SerializeItemType::Value, false);
                (Self::WITH_UNNAMED.ser)(item, writer)
            } else {
                writer.write_byte(0)
            }
        },
        |mut item, reader| {
            if let [0, ..] = reader.remaining() {
                item = item.scoped(Partial::set_default)?;
                Ok(item)
            } else {
                item = item.scoped(Partial::begin_some)?;
                (Self::WITH_UNNAMED.de_owned)(item, reader)
            }
        },
        |mut item, reader| {
            if let [0, ..] = reader.remaining() {
                item = item.scoped(Partial::set_default)?;
                Ok(item)
            } else {
                item = item.scoped(Partial::begin_some)?;
                (Self::WITH_UNNAMED.de_owned_borrow)(item, reader)
            }
        },
    );

    /// A facet deserializer for this [`IndexCore`].
    ///
    /// # Errors
    ///
    /// Returns an error if the NBT cannot be read.
    #[cfg(feature = "froglight-facet")]
    fn deserialize_named<'facet, const BORROW: bool>(
        _item: DeserializeItem<'facet, BORROW>,
        _reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        Err(ReaderError::from_string("This `IndexedNbt::IndexCore` does not support Facet!".into()))
    }

    /// A facet serializer for this [`IndexCore`].
    ///
    /// # Errors
    ///
    /// Returns an error if the NBT cannot be written.
    #[cfg(feature = "froglight-facet")]
    fn serialize_named(
        _item: SerializeItem<'_, '_>,
        _writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        Err(WriterError::from_string("This `IndexedNbt::IndexCore` does not support Facet!".into()))
    }

    /// A facet deserializer for this [`IndexCore`].
    ///
    /// # Errors
    ///
    /// Returns an error if the NBT cannot be read.
    #[cfg(feature = "froglight-facet")]
    fn deserialize_unnamed<'facet, const BORROW: bool>(
        _item: DeserializeItem<'facet, BORROW>,
        _reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        Err(ReaderError::from_string("This `IndexedNbt::IndexCore` does not support Facet!".into()))
    }

    /// A facet serializer for this [`IndexCore`].
    ///
    /// # Errors
    ///
    /// Returns an error if the NBT cannot be written.
    #[cfg(feature = "froglight-facet")]
    fn serialize_unnamed(
        _item: SerializeItem<'_, '_>,
        _writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        Err(WriterError::from_string("This `IndexedNbt::IndexCore` does not support Facet!".into()))
    }
}

impl<T: IndexCore<Mut>> IndexCore<Ref> for T {
    #[inline]
    fn root(&self) -> &[u8] { self.root() }

    #[inline]
    fn entries(&self) -> &[EntryIndex] { self.entries() }

    #[inline]
    unsafe fn entry_range(&self, index: usize) -> &[EntryIndex] {
        // SAFETY: The caller ensures that this is safe.
        unsafe { self.entry_range(index) }
    }

    #[inline]
    fn root_mut(&mut self) -> &mut [u8]
    where
        Ref: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>,
    {
        unreachable!("Cannot get mutable access with `Ref`!")
    }

    #[inline]
    unsafe fn entries_mut(&mut self) -> &mut [EntryIndex]
    where
        Ref: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>,
    {
        unreachable!("Cannot get mutable access with `Ref`!")
    }

    #[inline]
    unsafe fn entry_range_mut(&mut self, _index: usize) -> &mut [EntryIndex]
    where
        Ref: for<'a> NbtAccess<SLICE<'a> = &'a mut [u8]>,
    {
        unreachable!("Cannot get mutable access with `Ref`!")
    }
}

// -------------------------------------------------------------------------------------------------

/// A trait for either [`Ref`] or [`Mut`] access.
pub trait NbtAccess: Debug + Default + Copy + Eq + Hash + sealed::Sealed + 'static {
    /// The type of slice that NBT data is accessed through.
    type SLICE<'data>: Deref<Target = [u8]> + 'data;
    /// The type of reference that the core is accessed through.
    type CORE<'a, C: ?Sized + 'a>: Deref<Target = C> + Sized + 'a;

    /// Convert [`Self::CORE`] into a reference.
    fn into_core<C: ?Sized>(core: Self::CORE<'_, C>) -> &C;

    /// Convert [`Self::SLICE`] into a reference.
    fn into_ref(slice: Self::SLICE<'_>) -> &[u8];
}

/// A marker type for read-only access.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Ref;
impl NbtAccess for Ref {
    type CORE<'a, C: ?Sized + 'a> = &'a C;
    type SLICE<'data> = &'data [u8];

    fn into_core<C: ?Sized>(core: Self::CORE<'_, C>) -> &C { core }

    fn into_ref(slice: Self::SLICE<'_>) -> &[u8] { slice }
}

/// A marker type for mutable access.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Mut;
impl NbtAccess for Mut {
    type CORE<'a, C: ?Sized + 'a> = &'a mut C;
    type SLICE<'data> = &'data mut [u8];

    fn into_core<C: ?Sized>(core: Self::CORE<'_, C>) -> &C { core }

    fn into_ref(slice: Self::SLICE<'_>) -> &[u8] { slice }
}

// -------------------------------------------------------------------------------------------------

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::Ref {}
    impl Sealed for super::Mut {}
}
