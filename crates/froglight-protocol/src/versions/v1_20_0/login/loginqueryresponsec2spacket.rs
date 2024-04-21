use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct LoginQueryResponseC2SPacket {
    #[frog(var)]
    pub id: u32,
    pub data: Option<UnsizedBuffer>,
}
