//! TODO

use core::marker::PhantomData;

use facet::Facet;
use froglight_facet::facet::{WithFnAttr, prelude::*};
use froglight_facet_iter::deserialize::Deserializer;

use crate::types::indexed::core::{IndexCore, IndexedNbtSlice};

/// A [`FacetTemplate`] that serializes and deserializes the type as NBT.
///
/// # Example
///
/// ```rust
/// use facet::*;
/// use froglight_facet::prelude::*;
/// use froglight_nbt::prelude::*;
///
/// #[derive(Facet)]
/// #[facet(mc::with = NbtTemplate::<MyStruct>::WITH_NAMED)]
/// pub struct MyStruct {
///     inner: u32,
/// }
/// ```
pub struct NbtTemplate<'facet, T: Facet<'facet>>(PhantomData<&'facet T>);

impl<'facet, T: Facet<'facet>> NbtTemplate<'facet, T> {
    /// A [`WithFnAttr`] to be used with
    /// `#[derive(Facet)]` in a `#[facet(mc::with = ...)]`
    /// attribute.
    ///
    /// See [`FacetTemplate`] for more details and an example.
    pub const WITH_NAMED: WithFnAttr = Named::<'facet, T>::WITH_BORROW;
    /// A [`WithFnAttr`] to be used with
    /// `#[derive(Facet)]` in a `#[facet(mc::with = ...)]`
    /// attribute.
    ///
    /// See [`FacetTemplate`] for more details and an example.
    pub const WITH_UNNAMED: WithFnAttr = Unnamed::<'facet, T>::WITH_BORROW;
}

// -------------------------------------------------------------------------------------------------

struct Named<'facet, T: Facet<'facet>>(PhantomData<&'facet T>);

impl<'_f, T: Facet<'_f>> FacetTemplate for Named<'_f, T> {
    fn serialize(
        _item: SerializeItem<'_, '_>,
        _writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        todo!()
    }

    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        let nbt = IndexedNbtSlice::new_named(reader.remaining())
            .map_err(|()| ReaderError::from_str("Invalid NBT"))?;
        reader.consume(nbt.core().root().len())?;

        item.scoped(|partial| {
            let mut core = super::deserialize::deserialize_owned_core(&nbt);
            let de = Deserializer::new(partial, false, &mut core, Some("nbt"));

            de.complete().map_err(|err| {
                ReaderError::from_string(alloc::format!("Failed to deserialize NBT: {err}"))
            })
        })
    }
}

impl<'_f, T: Facet<'_f>> FacetBorrowedTemplate for Named<'_f, T> {
    fn deserialize_borrowed<'facet>(
        item: DeserializeItem<'facet, true>,
        reader: &mut Reader<'facet>,
    ) -> Result<DeserializeItem<'facet, true>, ReaderError> {
        let nbt = IndexedNbtSlice::new_named(reader.remaining())
            .map_err(|()| ReaderError::from_str("Invalid NBT"))?;
        reader.consume(nbt.core().root().len())?;

        item.scoped(|partial| {
            let mut core = super::deserialize::deserialize_borrowed_core(&nbt);
            let de = Deserializer::new(partial, true, &mut core, Some("nbt"));

            de.complete().map_err(|err| {
                ReaderError::from_string(alloc::format!("Failed to deserialize NBT: {err}"))
            })
        })
    }
}

// -------------------------------------------------------------------------------------------------

struct Unnamed<'facet, T: Facet<'facet>>(PhantomData<&'facet T>);

impl<'_f, T: Facet<'_f>> FacetTemplate for Unnamed<'_f, T> {
    fn serialize(
        _item: SerializeItem<'_, '_>,
        _writer: &mut Writer<'_>,
    ) -> Result<(), WriterError> {
        todo!()
    }

    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        let nbt = IndexedNbtSlice::new_unnamed(reader.remaining())
            .map_err(|()| ReaderError::from_str("Invalid NBT"))?;
        reader.consume(nbt.core().root().len())?;

        item.scoped(|partial| {
            let mut core = super::deserialize::deserialize_owned_core(&nbt);
            let de = Deserializer::new(partial, false, &mut core, Some("nbt"));

            de.complete().map_err(|err| {
                ReaderError::from_string(alloc::format!("Failed to deserialize NBT: {err}"))
            })
        })
    }
}

impl<'_f, T: Facet<'_f>> FacetBorrowedTemplate for Unnamed<'_f, T> {
    fn deserialize_borrowed<'facet>(
        item: DeserializeItem<'facet, true>,
        reader: &mut Reader<'facet>,
    ) -> Result<DeserializeItem<'facet, true>, ReaderError> {
        let nbt = IndexedNbtSlice::new_unnamed(reader.remaining())
            .map_err(|()| ReaderError::from_str("Invalid NBT"))?;
        reader.consume(nbt.core().root().len())?;

        item.scoped(|partial| {
            let mut core = super::deserialize::deserialize_borrowed_core(&nbt);
            let de = Deserializer::new(partial, true, &mut core, Some("nbt"));

            de.complete().map_err(|err| {
                ReaderError::from_string(alloc::format!("Failed to deserialize NBT: {err}"))
            })
        })
    }
}
