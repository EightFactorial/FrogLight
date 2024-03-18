use froglight_macros::FrogReadWrite;

use crate::common::PlayerHand;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct OpenWrittenBookS2CPacket {
    pub hand: PlayerHand,
}
