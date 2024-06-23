use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct SlotChangedStatePacket {
    #[frog(var)]
    pub slot_id: u32,
    #[frog(var)]
    pub container_id: u32,
    pub slot_state: bool,
}
