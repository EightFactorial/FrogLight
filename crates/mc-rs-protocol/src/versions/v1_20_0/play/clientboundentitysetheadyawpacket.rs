use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0])]
pub struct ClientboundEntitySetHeadYawPacket {
    pub entity_id: EntityId,
    pub yaw: i8,
}
