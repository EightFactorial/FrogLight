use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct LoginQueryRequestS2CPacket {
    #[frog(var)]
    pub id: u32,
    pub data: Option<UnsizedBuffer>,
}
