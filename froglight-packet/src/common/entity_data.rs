//! TODO
#![allow(missing_docs, reason = "TODO")]

use froglight_common::entity::EntityUuid;

use crate::common::{entity_id::VarEntityId, unsized_buffer::UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct EntityData {
    pub entity_id: VarEntityId,
    pub entity_uuid: EntityUuid,
    pub data: UnsizedBuffer<'static>,
}
