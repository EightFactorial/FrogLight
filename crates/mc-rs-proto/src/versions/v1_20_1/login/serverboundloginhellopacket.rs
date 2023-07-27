use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundLoginHelloPacket {
    pub username: String,
    pub uuid: Option<Uuid>,
}

#[test]
fn test_packet() {
    use crate::buffer::Encode;

    let mut buf = Vec::new();
    let packet = ServerboundLoginHelloPacket {
        username: "Username".to_string(),
        uuid: None,
    };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![8, 85, 115, 101, 114, 110, 97, 109, 101, 0]);
}
