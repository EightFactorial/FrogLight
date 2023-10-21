use crate::types::Vec3;
use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::types::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1])]
pub struct ClientboundPlayerSpawnPacket {
    pub entity_id: EntityId,
    pub uuid: Uuid,
    pub position: Vec3,
    pub yaw: i8,
    pub pitch: i8,
}
