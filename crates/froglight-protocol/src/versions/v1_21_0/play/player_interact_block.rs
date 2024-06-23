use froglight_macros::FrogReadWrite;

use crate::{common::InteractionHand, packet::BlockHit};

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlayerInteractBlockPacket {
    pub hand: InteractionHand,
    pub block_hit: BlockHit,
    #[frog(var)]
    pub sequence: u32,
}
