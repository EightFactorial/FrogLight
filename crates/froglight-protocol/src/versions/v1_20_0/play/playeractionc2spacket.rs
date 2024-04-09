use froglight_macros::FrogReadWrite;

use crate::{
    common::{BlockPosition, Direction},
    packet::ClientPlayerAction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2])]
pub struct PlayerActionC2SPacket {
    pub action: ClientPlayerAction,
    pub pos: BlockPosition,
    pub direction: Direction,
    #[frog(var)]
    pub sequence: u32,
}
