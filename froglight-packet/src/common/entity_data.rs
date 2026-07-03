//! TODO
#![allow(missing_docs, reason = "TODO")]

use froglight_entity::prelude::{EntityId, EntityUuid};
#[cfg(feature = "facet")]
use froglight_entity::{
    entity::{DataSetSerializer, EntityDataSet},
    prelude::EntityVersion,
};
#[cfg(feature = "facet")]
use froglight_facet::{self as mc, deserialize::DeserializeError};

use crate::common::{lpdvec3::LpDVec3, unsized_buffer::UnsizedBuffer};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct AddEntityBundle {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub entity_id: EntityId,
    pub entity_uuid: EntityUuid,
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub entity_type: u32,
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub velocity: LpDVec3,
    pub pitch: i8,
    pub yaw: i8,
    pub head_yaw: i8,
    pub data: UnsizedBuffer<'static>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", facet(opaque))]
pub struct SetEntityBundle {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    entity_id: EntityId,
    raw_data: UnsizedBuffer<'static>,
    #[cfg(feature = "facet")]
    fn_ptr: fn(&[u8]) -> Result<EntityDataSet<'static>, DeserializeError>,
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
    #[cfg(feature = "facet")]
    pub const fn new<V: EntityVersion>(
        entity_id: EntityId,
        raw_data: UnsizedBuffer<'static>,
    ) -> Self {
        Self::new_using(entity_id, raw_data, |slice| {
            froglight_facet::from_slice::<DataSetSerializer<V>>(slice)
                .map(DataSetSerializer::into_inner)
        })
    }

    /// Create a new [`SetEntityBundle`] with the given entity ID, raw data, and
    /// deserialization function.
    #[inline]
    #[must_use]
    #[cfg(feature = "facet")]
    pub const fn new_using(
        entity_id: EntityId,
        raw_data: UnsizedBuffer<'static>,
        fn_ptr: fn(&[u8]) -> Result<EntityDataSet<'static>, DeserializeError>,
    ) -> Self {
        Self { entity_id, raw_data, fn_ptr }
    }

    /// Get the [`EntityId`] of who to apply this [`EntityDataSet`] to.
    #[inline]
    #[must_use]
    pub const fn entity_id(&self) -> EntityId { self.entity_id }

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
    #[cfg(feature = "facet")]
    pub fn parse(&self) -> Result<EntityDataSet<'static>, DeserializeError> {
        (self.fn_ptr)(self.as_raw_data())
    }
}
