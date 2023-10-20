use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ClientboundEntityRotateAndMoveRelativePacket {
    pub entity_id: EntityId,
    pub delta_x: u16,
    pub delta_y: u16,
    pub delta_z: u16,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
