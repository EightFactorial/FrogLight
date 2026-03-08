use alloc::vec::Vec;
use core::marker::PhantomData;

use facet::Facet;
#[cfg(feature = "facet")]
use facet::{Partial, Peek};
use facet_minecraft::replace_with::replace_with_or_abort;
#[cfg(feature = "facet")]
use facet_minecraft::{
    self as mc, DeserializeFn, SerializeFn,
    deserialize::{InputCursor, error::DeserializeValueError},
    serialize::{buffer::SerializeWriter, error::SerializeIterError},
};

use crate::{entity::EntityDataSet, version::EntityVersion};

/// A serializer and deserializer for [`EntityDataSet`]s.
///
/// Requires the `facet` feature to be enabled.
#[derive(Debug, Clone, PartialEq, Facet)]
#[facet(opaque)]
#[facet(mc::serialize = DataSetSerializer::<V>::SERIALIZE)]
#[facet(mc::deserialize = DataSetSerializer::<V>::DESERIALIZE)]
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

impl<V: EntityVersion> DataSetSerializer<V> {
    const DESERIALIZE: DeserializeFn =
        DeserializeFn::new(Self::facet_deserialize, Self::facet_deserialize);
    const SERIALIZE: SerializeFn = SerializeFn::new(Self::facet_serialize);

    fn facet_deserialize<'facet, const BORROW: bool>(
        partial: &mut Partial<'facet, BORROW>,
        cursor: &mut InputCursor<'_, 'facet>,
    ) -> Result<(), DeserializeValueError> {
        let mut list = Vec::new();
        loop {
            let id = cursor.take(1)?[0];
            if id == 0xff {
                break;
            }
            list.push((id, (V::DATATYPE_DESERIALIZE)(cursor)?));
        }

        let serializer = Self::new(EntityDataSet::new(list));
        replace_with_or_abort(partial, |partial| partial.set(serializer).unwrap());
        Ok(())
    }

    fn facet_serialize<'input, 'facet>(
        peek: Peek<'input, 'facet>,
        buffer: &mut dyn SerializeWriter,
    ) -> Result<(), SerializeIterError<'input, 'facet>> {
        let serializer = peek.get::<Self>()?;
        for (id, val) in serializer.dataset.to_ref() {
            if !buffer.write_data(&[*id]) {
                return Err(SerializeIterError::new());
            }
            (V::DATATYPE_SERIALIZE)(&(), val, buffer)?;
        }
        if buffer.write_data(&[0xff]) { Ok(()) } else { Err(SerializeIterError::new()) }
    }
}
