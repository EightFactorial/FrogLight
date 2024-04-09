use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct HealthUpdateS2CPacket {
    pub health: f32,
    #[frog(var)]
    pub food: u32,
    pub saturation: f32,
}
