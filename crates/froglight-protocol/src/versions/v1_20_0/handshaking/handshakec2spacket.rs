use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::ConnectionIntent;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [251, 5, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct HandshakeC2SPacket {
    #[frog(var)]
    pub protocol_version: i32,
    pub hostname: CompactString,
    pub port: u16,
    pub intention: ConnectionIntent,
}
