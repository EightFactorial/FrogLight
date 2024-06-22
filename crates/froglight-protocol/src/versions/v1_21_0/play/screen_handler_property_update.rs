use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 1, 0, 1])]
pub struct ScreenHandlerPropertyUpdatePacket {
    pub container_id: u8,
    pub property_id: u16,
    pub value: u16,
}
