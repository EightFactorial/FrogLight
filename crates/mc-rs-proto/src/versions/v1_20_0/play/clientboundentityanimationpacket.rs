use mc_rs_macros::Transcode;

use crate::types::{packets::animation::AnimationAction, EntityId};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityAnimationPacket {
    pub entity_id: EntityId,
    pub animation: AnimationAction,
}
