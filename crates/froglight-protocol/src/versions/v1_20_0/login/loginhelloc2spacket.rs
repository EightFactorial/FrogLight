use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [8, 85, 115, 101, 114, 110, 97, 109, 101, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct LoginHelloC2SPacket {
    pub username: CompactString,
    pub uuid: Option<Uuid>,
}
