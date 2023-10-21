use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 1])]
pub struct ServerboundQueryEntityNbtPacket {
    #[var]
    pub request_id: u32,
    pub entity_id: EntityId,
}
