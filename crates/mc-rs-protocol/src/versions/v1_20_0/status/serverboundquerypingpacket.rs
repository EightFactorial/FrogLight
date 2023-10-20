use std::time::{SystemTime, UNIX_EPOCH};

use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 1])]

pub struct ServerboundQueryPingPacket(u64);

impl ServerboundQueryPingPacket {
    pub fn unix_epoch() -> Self {
        Self(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }
}
