use froglight_macros::FrogReadWrite;

use crate::common::ResourceKey;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub struct FeaturesS2CPacket {
    pub features: Vec<ResourceKey>,
}
