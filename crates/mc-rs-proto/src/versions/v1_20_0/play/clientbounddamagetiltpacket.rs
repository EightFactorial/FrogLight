use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundDamageTiltPacket {
    pub entity_id: EntityId,
    pub yaw: f32,
}
