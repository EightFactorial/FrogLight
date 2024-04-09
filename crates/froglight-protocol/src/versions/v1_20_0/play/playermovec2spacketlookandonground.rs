use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct PlayerMoveC2SPacketLookAndOnGround {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
