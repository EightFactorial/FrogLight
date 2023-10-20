use azalea_chat::FormattedText;
use mc_rs_macros::Transcode;

use crate::types::EntityId;

// TODO: Verify that this is FormattedText and not String
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundDeathMessagePacket {
    pub player_id: EntityId,
    pub message: FormattedText,
}
