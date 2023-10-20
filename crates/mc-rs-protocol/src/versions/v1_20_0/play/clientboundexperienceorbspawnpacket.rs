use mc_rs_macros::Transcode;

use crate::types::{EntityId, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16])]
pub struct ClientboundExperienceOrbSpawnPacket {
    pub entity_id: EntityId,
    pub position: Vec3,
    pub value: u16,
}
