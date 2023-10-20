use crate::types::Vec3;
use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])]
pub struct ClientboundEntityPositionPacket {
    pub entity_id: EntityId,
    pub position: Vec3,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
