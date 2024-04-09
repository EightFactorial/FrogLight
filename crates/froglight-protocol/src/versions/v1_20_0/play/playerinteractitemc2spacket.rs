use froglight_macros::FrogReadWrite;

use crate::common::InteractionHand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0])]
pub struct PlayerInteractItemC2SPacket {
    pub hand: InteractionHand,
    #[frog(var)]
    pub sequence: u32,
}
