use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct OpenHorseScreenPacket {
    pub container_id: u8,
    #[frog(var)]
    pub slot_count: u32,
    pub horse_id: u32,
}
