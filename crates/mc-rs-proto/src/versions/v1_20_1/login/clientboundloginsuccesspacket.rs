use mc_rs_macros::Transcode;

use crate::types::GameProfile;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLoginSuccessPacket {
    pub profile: GameProfile,
}

#[test]
fn test_packet() {
    use crate::buffer::Encode;
    use uuid::Uuid;

    let mut buf = Vec::new();
    let packet = ClientboundLoginSuccessPacket {
        profile: GameProfile {
            uuid: Uuid::from_u128(0x8002_0000_0000_0000_0000_0000_0000_0000),
            name: "Herobrine".to_string(),
            properties: Default::default(),
        },
    };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(
        buf,
        vec![
            128, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 72, 101, 114, 111, 98, 114, 105,
            110, 101, 0
        ]
    );
}
