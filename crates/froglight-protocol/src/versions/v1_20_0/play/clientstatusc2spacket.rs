use froglight_macros::FrogReadWrite;

use crate::common::ClientStatusAction;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ClientStatusC2SPacket(pub ClientStatusAction);
