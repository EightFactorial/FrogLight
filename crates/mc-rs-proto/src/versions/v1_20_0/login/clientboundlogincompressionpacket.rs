use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLoginCompressionPacket {
    #[var]
    pub threshold: i32,
}

#[test]
fn test_packet() {
    use crate::buffer::Encode;

    let mut buf = Vec::new();
    let packet = ClientboundLoginCompressionPacket { threshold: 256 };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![128, 2]);
}
