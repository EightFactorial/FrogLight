use mc_rs_macros::Transcode;

use crate::types::enums::ConnectionIntent;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundHandshakePacket {
    #[var]
    pub protocol_version: i32,
    pub hostname: String,
    pub port: u16,
    pub intention: ConnectionIntent,
}

#[test]
fn test_packet() {
    use crate::{buffer::Encode, versions::v1_20_1::V1_20_1, Version};

    let mut buf = Vec::new();

    let packet = ServerboundHandshakePacket {
        protocol_version: V1_20_1::ID,
        hostname: "localhost".to_string(),
        port: 25565,
        intention: ConnectionIntent::Status,
    };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(
        buf,
        vec![251, 5, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1]
    );
}
