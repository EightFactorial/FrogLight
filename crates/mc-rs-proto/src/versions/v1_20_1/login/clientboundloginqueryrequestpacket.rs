use crate::types::UnsizedByteBuffer;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLoginQueryRequestPacket {
    #[var]
    pub id: u32,
    pub data: Option<UnsizedByteBuffer>,
}

#[test]
fn test_packet() {
    use crate::buffer::Encode;

    let mut buf = Vec::new();
    let packet = ClientboundLoginQueryRequestPacket { id: 0, data: None };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0]);
    buf.clear();

    let packet = ClientboundLoginQueryRequestPacket {
        id: 1,
        data: Some(UnsizedByteBuffer::from_bytes(vec![1, 2, 3, 4])),
    };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![1, 1, 1, 2, 3, 4]);
}
