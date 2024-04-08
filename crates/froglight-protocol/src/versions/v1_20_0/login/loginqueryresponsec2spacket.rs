use froglight_macros::FrogReadWrite;

use crate::common::{ResourceKey, UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct LoginQueryResponseC2SPacket {
    #[frog(var)]
    pub id: u32,
    pub identifier: ResourceKey,
    pub data: UnsizedBuffer,
}
