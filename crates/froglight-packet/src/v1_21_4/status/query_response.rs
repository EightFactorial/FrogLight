#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};

use crate::common::ServerStatus;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct QueryResponsePacket {
    #[cfg_attr(feature = "io", frog(json))]
    pub status: ServerStatus,
}
