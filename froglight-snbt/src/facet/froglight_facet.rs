//! TODO

use core::marker::PhantomData;

use facet::Facet;
use froglight_facet::facet::prelude::*;
use froglight_facet_iter::deserialize::Deserializer;

use crate::types::indexed::core::IndexedSnbtSlice;

/// A [`FacetTemplate`] that serializes and deserializes the type as SNBT.
///
/// # Example
///
/// ```rust
/// use facet::*;
/// use froglight_facet::prelude::*;
/// use froglight_snbt::prelude::*;
///
/// #[derive(Facet)]
/// #[facet(mc::with = SnbtTemplate::<MyStruct>::WITH)]
/// pub struct MyStruct {
///     inner: u32,
/// }
/// ```
pub struct SnbtTemplate<'facet, T: Facet<'facet>>(PhantomData<&'facet T>);

impl<'facet, T: Facet<'facet>> SnbtTemplate<'facet, T> {
    /// A [`WithFnAttr`] to be used with
    /// `#[derive(Facet)]` in a `#[facet(mc::with = ...)]`
    /// attribute.
    ///
    /// See [`FacetTemplate`] for more details and an example.
    pub const WITH: WithFnAttr = Self::WITH_BORROW;
}

// -------------------------------------------------------------------------------------------------

impl<'_f, T: Facet<'_f>> FacetTemplate for SnbtTemplate<'_f, T> {
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
        let len = decode_u32_from(reader)? as usize;
        let content = reader.read(len)?;
        let content = str::from_utf8(content).map_err(ReaderError::other)?;

        let snbt = IndexedSnbtSlice::new_ref(content)
            .map_err(|()| ReaderError::from_str("Invalid SNBT"))?;

        item.scoped(|partial| {
            let mut core = super::deserialize::deserialize_owned_core(&snbt);
            let de = Deserializer::new(partial, false, &mut core, Some("snbt"));

            de.complete().map_err(|err| {
                ReaderError::from_string(alloc::format!("Failed to deserialize SNBT: {err}"))
            })
        })
    }
}

impl<'_f, T: Facet<'_f>> FacetBorrowedTemplate for SnbtTemplate<'_f, T> {
    fn deserialize_borrowed<'facet>(
        item: DeserializeItem<'facet, true>,
        reader: &mut Reader<'facet>,
    ) -> Result<DeserializeItem<'facet, true>, ReaderError> {
        let len = decode_u32_from(reader)? as usize;
        let content = str::from_utf8(reader.read(len)?).map_err(ReaderError::other)?;

        let snbt = IndexedSnbtSlice::new_ref(content)
            .map_err(|()| ReaderError::from_str("Invalid SNBT"))?;

        item.scoped(|partial| {
            let mut core = super::deserialize::deserialize_borrowed_core(&snbt);
            let de = Deserializer::new(partial, false, &mut core, Some("snbt"));

            de.complete().map_err(|err| {
                ReaderError::from_string(alloc::format!("Failed to deserialize SNBT: {err}"))
            })
        })
    }
}
