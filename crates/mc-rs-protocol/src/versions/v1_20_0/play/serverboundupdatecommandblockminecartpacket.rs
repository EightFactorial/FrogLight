use compact_str::CompactString;
use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 1])]
pub struct ServerboundUpdateCommandBlockMinecartPacket {
    pub entity_id: EntityId,
    pub command: CompactString,
    pub track_output: bool,
}
