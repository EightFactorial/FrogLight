use mc_rs_macros::Transcode;

use crate::types::EntityId;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundQueryEntityNbtPacket {
    #[var]
    pub id: u32,
    pub entity_id: EntityId,
}
