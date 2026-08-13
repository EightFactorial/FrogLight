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
/// use froglight_nbt::facet::with::SnbtTemplate;
///
/// #[derive(Facet)]
/// #[facet(mc::with = SnbtTemplate::<MyStruct>::WITH)]
/// pub struct MyStruct {
///     inner: u32,
/// }
/// ```
pub struct SnbtTemplate<'facet, T: Facet<'facet>>(PhantomData<&'facet T>);

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
        let content = reader.read(len)?;
        let content = str::from_utf8(content).map_err(ReaderError::other)?;

        item.scoped(|partial| {
            let snbt = IndexedSnbtSlice::new_ref(content)
                .map_err(|()| ReaderError::from_str("Invalid SNBT"))?;

            // TODO: Do some lifetime trickery and use `deserialize_borrowed_core`.
            let mut core = super::deserialize::deserialize_owned_core(&snbt);
            let de = Deserializer::new(partial, false, &mut core, Some("snbt"));

            de.complete().map_err(|err| {
                ReaderError::from_string(alloc::format!("Failed to deserialize SNBT: {err}"))
            })
        })
    }
}
