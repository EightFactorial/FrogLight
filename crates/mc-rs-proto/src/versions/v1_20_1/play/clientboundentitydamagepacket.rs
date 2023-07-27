use mc_rs_macros::Transcode;

use crate::types::{position::Vec3, EntityId, NonZeroOption};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityDamagePacket {
    pub entity_id: EntityId,
    #[var]
    pub damage_type: u32,
    pub source_cause: NonZeroOption<EntityId>,
    pub source_direct: NonZeroOption<EntityId>,
    pub position: Option<Vec3>,
}
