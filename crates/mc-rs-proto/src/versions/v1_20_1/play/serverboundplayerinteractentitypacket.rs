use mc_rs_macros::Transcode;

use crate::types::{packets::interaction::InteractionAction, EntityId};

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerInteractEntityPacket {
    pub entity_id: EntityId,
    pub action: InteractionAction,
    pub secondary: bool,
}
