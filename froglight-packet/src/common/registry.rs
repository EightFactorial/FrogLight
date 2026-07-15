//! TODO
#![allow(missing_docs, reason = "WIP")]

#[cfg(feature = "facet")]
use facet::Peek;
use froglight_common::prelude::Identifier;
#[cfg(feature = "facet")]
#[allow(clippy::wildcard_imports, reason = "Readability")]
use froglight_facet::{self as mc, facet::template::*};
use froglight_nbt::types::indexed::alloc::IndexedNbtCow;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct RegistryDataEntry {
    pub identifier: Identifier<'static>,
    #[cfg_attr(feature = "facet", facet(mc::with = RegistryDataEntry::WITH))]
    pub nbt: Option<IndexedNbtCow<'static>>,
}

#[cfg(feature = "facet")]
impl FacetTemplate for RegistryDataEntry {
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError> {
        let item = item.get::<Option<IndexedNbtCow<'_>>>()?;
        if let Some(nbt) = item {
            writer.write_byte(1)?;

            let inner = SerializeItem::new(Peek::new(nbt), SerializeItemType::Value, false);
            IndexedNbtCow::WITH_UNNAMED.serialize(inner, writer)
        } else {
            writer.write_byte(0)
        }
    }

    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        match reader.get_byte()? {
            0 => item.set::<Option<IndexedNbtCow<'facet>>>(None),
            1 => item.scoped(|mut partial| {
                partial = partial.begin_some()?;

                let mut item = DeserializeItem::new(partial, DeserializeDesc::new(false, None));
                item = IndexedNbtCow::WITH_UNNAMED.deserialize(item, reader)?;

                Ok(item.into_inner().0)
            }),
            unk => Err(ReaderError::InvalidBool(unk)),
        }
    }
}
