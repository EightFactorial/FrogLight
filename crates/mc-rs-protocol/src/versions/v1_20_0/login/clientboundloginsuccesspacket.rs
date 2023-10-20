use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

use crate::types::GameProfile;

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
pub struct ClientboundLoginSuccessPacket(GameProfile);

#[test]
fn clientbound_login_success_packet_test() {
    use compact_str::CompactString;
    use uuid::Uuid;

    use crate::buffer::Encode;

    let mut buf = Vec::new();
    let packet = ClientboundLoginSuccessPacket(GameProfile {
        uuid: Uuid::from_u128(0x8002_0000_0000_0000_0000_0000_0000_0000),
        name: CompactString::from("Herobrine"),
        properties: Default::default(),
    });

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(
        buf,
        vec![
            128, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 72, 101, 114, 111, 98, 114, 105,
            110, 101, 0
        ]
    );
}
