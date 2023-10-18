use mc_rs_macros::Transcode;

use crate::types::{EntityId, Vec3};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundExperienceOrbSpawnPacket {
    pub entity_id: EntityId,
    pub position: Vec3,
    pub value: u16,
}
