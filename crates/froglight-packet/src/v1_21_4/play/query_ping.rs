#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct QueryPingPacket {
    pub ping: u64,
}

#[cfg(feature = "std")]
impl QueryPingPacket {
    /// Create a new [`QueryPingPacket`] with the current time in milliseconds
    /// since the [`UNIX_EPOCH`].
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::missing_panics_doc)]
    pub fn unix_epoch() -> Self {
        Self::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
    }
}
