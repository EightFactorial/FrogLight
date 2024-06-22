use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;
use serde::{Deserialize, Serialize};

use crate::packet::ServerStatus;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, Serialize, Deserialize, FrogReadWrite,
)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct QueryResponsePacket {
    pub status: ServerStatus,
}
