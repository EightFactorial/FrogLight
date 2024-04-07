use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct BundleS2CPacket;
