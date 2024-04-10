use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use crate::common::EntityUuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct SpectatorTeleportC2SPacket {
    pub target_uuid: EntityUuid,
}
