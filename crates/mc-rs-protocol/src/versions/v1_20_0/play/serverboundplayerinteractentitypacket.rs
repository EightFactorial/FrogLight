use mc_rs_macros::Transcode;

use crate::types::{packets::interaction::InteractionAction, EntityId};

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 1, 0])]
pub struct ServerboundPlayerInteractEntityPacket {
    pub entity_id: EntityId,
    pub action: InteractionAction,
    pub secondary: bool,
}
