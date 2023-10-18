use mc_rs_macros::Transcode;

use crate::types::packets::interaction::InteractionHand;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerInteractItemPacket {
    pub hand: InteractionHand,
    #[var]
    pub sequence: u32,
}
