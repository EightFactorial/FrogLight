use std::time::{SystemTime, UNIX_EPOCH};

use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite,
)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct QueryPingPacket {
    pub ping: u64,
}

impl QueryPingPacket {
    /// Create a new [`QueryPingPacket`] with the current time in milliseconds
    /// since the [`UNIX_EPOCH`].
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn unix_epoch() -> Self {
        Self::from(
            u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis())
                .unwrap_or_default(),
        )
    }
}
