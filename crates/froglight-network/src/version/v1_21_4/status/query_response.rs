#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogBuf;
use serde::{Deserialize, Serialize};

use crate::types::ServerStatus;

#[derive(
    Debug, Clone, PartialEq, Eq, FrogBuf, Serialize, Deserialize, Deref, DerefMut, From, Into,
)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub struct QueryResponsePacket {
    #[frog(json)]
    pub status: ServerStatus,
}
