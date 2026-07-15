use alloc::vec::Vec;
use core::marker::PhantomData;

use facet::Facet;
#[allow(clippy::wildcard_imports, reason = "Readability")]
use froglight_facet::{self as mc, facet::template::*};

use crate::{entity::EntityDataSet, version::EntityVersion};

/// A serializer and deserializer for [`EntityDataSet`]s.
///
/// Requires the `facet` feature to be enabled.
#[derive(Debug, Clone, PartialEq, Facet)]
#[facet(opaque, mc::with = DataSetSerializer::<V>::WITH)]
pub struct DataSetSerializer<V: EntityVersion> {
    dataset: EntityDataSet<'static>,
    _phantom: PhantomData<V>,
}

impl<V: EntityVersion> DataSetSerializer<V> {
    /// Create a new [`DataSetSerializer`] from the given [`EntityDataSet`].
    #[must_use]
    pub const fn new(dataset: EntityDataSet<'static>) -> Self {
        Self { dataset, _phantom: PhantomData }
    }

    /// Get a reference to the inner [`EntityBundle`].
    #[inline]
    #[must_use]
    pub const fn dataset(&self) -> &EntityDataSet<'static> { &self.dataset }

    /// Get a mutable reference to the inner [`EntityBundle`].
    #[inline]
    #[must_use]
    pub const fn dataset_mut(&mut self) -> &mut EntityDataSet<'static> { &mut self.dataset }

    /// Return the inner [`EntityBundle`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> EntityDataSet<'static> { self.dataset }
}

impl<V: EntityVersion> FacetTemplate for DataSetSerializer<V> {
    fn serialize(item: SerializeItem<'_, '_>, writer: &mut Writer<'_>) -> Result<(), WriterError> {
        let ser = item.get::<Self>()?;
        for (id, val) in ser.dataset.to_ref() {
            writer.write_byte(*id)?;

            (V::DATATYPE_SERIALIZE)(val, writer)?;
        }

        writer.write_byte(0xff)
    }

    fn deserialize<'facet, const BORROW: bool>(
        item: DeserializeItem<'facet, BORROW>,
        reader: &mut Reader<'_>,
    ) -> Result<DeserializeItem<'facet, BORROW>, ReaderError> {
        let mut list = Vec::new();
        loop {
            let id = reader.read_byte()?;
            #[cfg(feature = "tracing_ext")]
            tracing::trace!(target: "froglight_entity::entity", "EntityDataPos: {id:?}");

            if id == 0xff {
                break;
            }

            list.push((id, (V::DATATYPE_DESERIALIZE)(reader)?));
        }

        item.set(Self::new(EntityDataSet::new(list)))
    }
}
