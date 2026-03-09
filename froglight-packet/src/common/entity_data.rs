//! TODO
#![allow(missing_docs, reason = "TODO")]

#[cfg(feature = "facet")]
use facet_minecraft as mc;
#[cfg(feature = "std")]
use facet_minecraft::deserialize::error::DeserializeError;
use froglight_common::{entity::EntityUuid, prelude::EntityId};
use froglight_entity::entity::EntityDataSet;
#[cfg(feature = "std")]
use froglight_entity::{entity::DataSetSerializer, prelude::EntityVersion};

use crate::common::{entity_id::VarEntityId, unsized_buffer::UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct AddEntityBundle {
    pub entity_id: VarEntityId,
    pub entity_uuid: EntityUuid,
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub entity_type: u32,
    pub data: UnsizedBuffer<'static>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(opaque))]
pub struct SetEntityBundle {
    entity_id: VarEntityId,
    raw_data: UnsizedBuffer<'static>,
    fn_ptr: fn(&[u8]) -> Result<EntityDataSet<'static>, DeserializeError<'static>>,
}

impl Eq for SetEntityBundle {}
impl PartialEq for SetEntityBundle {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id && self.raw_data == other.raw_data
    }
}

impl SetEntityBundle {
    /// Create a new [`SetEntityBundle`] with the given entity ID and raw data.
    #[must_use]
    #[cfg(feature = "std")]
    pub const fn new<V: EntityVersion>(
        entity_id: VarEntityId,
        raw_data: UnsizedBuffer<'static>,
    ) -> Self {
        Self::new_using(entity_id, raw_data, |slice| {
            facet_minecraft::from_slice_owned::<DataSetSerializer<V>>(slice)
                .map(DataSetSerializer::into_inner)
        })
    }

    /// Create a new [`SetEntityBundle`] with the given entity ID, raw data, and
    /// deserialization function.
    #[inline]
    #[must_use]
    pub const fn new_using(
        entity_id: VarEntityId,
        raw_data: UnsizedBuffer<'static>,
        fn_ptr: fn(&[u8]) -> Result<EntityDataSet<'static>, DeserializeError<'static>>,
    ) -> Self {
        Self { entity_id, raw_data, fn_ptr }
    }

    /// Get the [`EntityId`] of who to apply this [`EntityDataSet`] to.
    #[inline]
    #[must_use]
    pub const fn entity_id(&self) -> EntityId { self.entity_id.0 }

    /// Get the raw data buffer of this [`SetEntityBundle`].
    #[inline]
    #[must_use]
    pub const fn as_raw_data(&self) -> &[u8] { self.raw_data.as_slice() }

    /// Parse the raw data of this [`SetEntityBundle`] into an
    /// [`EntityDataSet`].
    ///
    /// # Errors
    ///
    /// Returns an error if the data is invalid.
    #[inline]
    pub fn parse(&self) -> Result<EntityDataSet<'static>, DeserializeError<'static>> {
        (self.fn_ptr)(self.as_raw_data())
    }
}
