use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct RemoveMessagePacket {
    #[frog(var)]
    pub message_id: u32,
    pub signature: [u8; 256],
}
