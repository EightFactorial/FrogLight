use mc_rs_macros::Transcode;

use crate::types::{EntityId, NonZero, Vec3};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityDamagePacket {
    pub entity_id: EntityId,
    #[var]
    pub damage_type: u32,
    pub source_cause: NonZero<EntityId>,
    pub source_direct: NonZero<EntityId>,
    pub position: Option<Vec3>,
}
