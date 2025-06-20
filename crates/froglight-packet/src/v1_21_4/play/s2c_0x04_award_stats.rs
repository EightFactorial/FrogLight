//! This file is auto-generated. Disable this by adding an `@manual` tag.
//!
//! @manual @generated by {COMMIT_HASH}

use bevy_platform::collections::HashMap;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};

use crate::common::PlayerStatisticType;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct StatisticsS2CPacket {
    #[cfg_attr(feature = "io", frog(var))]
    pub statistics: HashMap<PlayerStatisticType, u32>,
}
