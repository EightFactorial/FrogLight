use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct CooldownUpdatePacket {
    #[frog(var)]
    pub item: u32,
    #[frog(var)]
    pub cooldown: u32,
}
