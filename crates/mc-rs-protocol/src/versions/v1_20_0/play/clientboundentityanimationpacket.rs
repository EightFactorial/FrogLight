use mc_rs_macros::Transcode;

use crate::types::{packets::animation::AnimationAction, EntityId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [42, 0])]
pub struct ClientboundEntityAnimationPacket {
    pub entity_id: EntityId,
    pub animation: AnimationAction,
}
