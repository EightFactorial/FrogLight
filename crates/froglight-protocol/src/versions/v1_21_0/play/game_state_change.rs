use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [3, 0, 0, 0, 0])]
pub struct GameStateChangePacket {
    pub event_id: u8,
    pub event_data: f32,
}
