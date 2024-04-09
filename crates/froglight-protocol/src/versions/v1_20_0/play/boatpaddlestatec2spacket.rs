use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0])]
pub struct BoatPaddleStateC2SPacket {
    pub left_paddling: bool,
    pub right_paddling: bool,
}
