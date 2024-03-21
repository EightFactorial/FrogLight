use froglight_macros::FrogReadWrite;

use crate::common::{BlockHit, InteractionHand};

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerInteractBlockC2SPacket {
    pub hand: InteractionHand,
    pub block_hit: BlockHit,
    #[frog(var)]
    pub sequence: u32,
}
