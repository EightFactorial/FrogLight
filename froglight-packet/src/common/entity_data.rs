//! TODO
#![allow(missing_docs, reason = "TODO")]

#[cfg(feature = "facet")]
use facet_minecraft as mc;
#[cfg(feature = "std")]
use facet_minecraft::deserialize::error::DeserializeError;
use froglight_common::entity::EntityUuid;
use froglight_entity::entity::EntityDataSet;
#[cfg(feature = "std")]
use froglight_entity::{entity::DataSetSerializer, prelude::EntityVersion};

use crate::common::{entity_id::VarEntityId, unsized_buffer::UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct AddedEntityData {
    pub entity_id: VarEntityId,
    pub entity_uuid: EntityUuid,
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub entity_type: u32,
    pub data: UnsizedBuffer<'static>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct SetEntityData {
    pub entity_id: VarEntityId,
    pub dataset: EntityDataSet<'static>,
}

impl SetEntityData {
    /// Attempt to parse an [`SetEntityData`] from the given buffer.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is not a valid entity data set.
    #[cfg(feature = "std")]
    pub fn try_from<V: EntityVersion>(
        entity_id: VarEntityId,
        buffer: UnsizedBuffer<'static>,
    ) -> Result<Self, DeserializeError<'static>> {
        Ok(Self {
            entity_id,
            dataset: facet_minecraft::from_slice_owned::<DataSetSerializer<V>>(buffer.as_slice())?
                .into_inner(),
        })
    }
}
