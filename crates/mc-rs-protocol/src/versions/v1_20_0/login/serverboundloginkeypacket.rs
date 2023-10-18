use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundLoginKeyPacket {
    pub key: Vec<u8>,
    pub challenge: Vec<u8>,
}

#[test]
fn test_packet() {
    use crate::buffer::Encode;

    let mut buf = Vec::new();
    let packet = ServerboundLoginKeyPacket {
        key: vec![8, 7, 6, 5, 4, 3, 2, 1],
        challenge: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(
        buf,
        vec![8, 8, 7, 6, 5, 4, 3, 2, 1, 8, 1, 2, 3, 4, 5, 6, 7, 8]
    )
}
