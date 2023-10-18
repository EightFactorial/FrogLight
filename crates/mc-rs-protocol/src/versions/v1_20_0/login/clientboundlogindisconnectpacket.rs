use azalea_chat::FormattedText;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLoginDisconnectPacket {
    pub reason: FormattedText,
}

#[test]
fn test_packet() {
    use crate::buffer::Encode;

    let mut buf = Vec::new();
    let packet = ClientboundLoginDisconnectPacket {
        reason: "Disconnect".into(),
    };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(
        String::from_utf8(buf[1..].to_vec()).unwrap(),
        r#"{"text":"Disconnect"}"#
    );
}
