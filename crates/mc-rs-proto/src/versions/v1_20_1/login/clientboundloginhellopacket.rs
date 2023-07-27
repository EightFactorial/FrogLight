use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLoginHelloPacket {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub key_nonce: Vec<u8>,
}

#[test]
fn test_packet() {
    use crate::buffer::Encode;

    let mut buf = Vec::new();
    let packet = ClientboundLoginHelloPacket {
        server_id: "server_id".to_string(),
        public_key: vec![8, 7, 6, 5, 4, 3, 2, 1],
        key_nonce: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(
        buf,
        vec![
            9, 115, 101, 114, 118, 101, 114, 95, 105, 100, 8, 8, 7, 6, 5, 4, 3, 2, 1, 8, 1, 2, 3,
            4, 5, 6, 7, 8
        ]
    )
}
