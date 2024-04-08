use std::time::{SystemTime, UNIX_EPOCH};

use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 1])]
pub struct QueryPongS2CPacket {
    pub time: u64,
}

impl QueryPongS2CPacket {
    /// Create a new [`QueryPongS2CPacket`] with the current time in seconds
    /// since the [`UNIX_EPOCH`].
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn unix_epoch() -> Self {
        Self::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
    }
}
