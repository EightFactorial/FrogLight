use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

use crate::common::ChunkBiomeData;

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite,
)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ChunkBiomeDataS2CPacket(pub Vec<ChunkBiomeData>);
