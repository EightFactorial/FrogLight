use std::time::{SystemTime, UNIX_EPOCH};

use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundQueryPingPacket {
    pub time: u64,
}

impl Default for ServerboundQueryPingPacket {
    fn default() -> Self {
        Self {
            // Current system time
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

#[test]
fn test_packet() {
    use crate::buffer::Encode;

    let mut buf = Vec::new();
    let packet = ServerboundQueryPingPacket { time: 1 };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 1]);
}
