use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0])]
pub struct ButtonClickC2SPacket {
    pub container_id: u8,
    pub button_id: u8,
}
