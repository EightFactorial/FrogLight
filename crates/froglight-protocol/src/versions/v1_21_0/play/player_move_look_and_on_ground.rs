use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct PlayerMoveLookAndOnGroundPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
