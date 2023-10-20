use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0])]
pub struct ClientboundDamageTiltPacket {
    pub entity_id: EntityId,
    pub yaw: f32,
}
