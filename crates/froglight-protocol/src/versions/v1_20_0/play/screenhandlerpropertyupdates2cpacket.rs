use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 1, 0, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ScreenHandlerPropertyUpdateS2CPacket {
    pub container_id: u8,
    pub property_id: u16,
    pub value: u16,
}
