use froglight_macros::FrogReadWrite;

use crate::common::InteractionHand;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct PlayerInteractItemPacket {
    pub hand: InteractionHand,
    #[frog(var)]
    pub sequence: u32,
    pub yaw: f32,
    pub pitch: f32,
}
