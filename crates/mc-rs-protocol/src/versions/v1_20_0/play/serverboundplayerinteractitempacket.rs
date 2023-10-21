use mc_rs_macros::Transcode;

use crate::types::packets::interaction::InteractionHand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0])]
pub struct ServerboundPlayerInteractItemPacket {
    pub hand: InteractionHand,
    #[var]
    pub sequence: u32,
}
