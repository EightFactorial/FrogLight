use mc_rs_macros::Transcode;

use crate::types::{position::Vec3, EntityId};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundExperienceOrbSpawnPacket {
    pub entity_id: EntityId,
    pub position: Vec3,
    pub value: u16,
}
