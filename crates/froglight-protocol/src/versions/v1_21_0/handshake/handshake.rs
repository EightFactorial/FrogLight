use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::ConnectionIntent;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [251, 5, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1])]
pub struct HandshakePacket {
    #[frog(var)]
    pub protocol: i32,
    pub address: CompactString,
    pub port: u16,
    pub intent: ConnectionIntent,
}
