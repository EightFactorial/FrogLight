use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateCommandBlockMinecartPacket {
    pub entity_id: EntityId,
    pub command: String,
    pub track_output: bool,
}
