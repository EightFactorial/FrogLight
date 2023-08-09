use mc_rs_macros::Transcode;

use crate::types::{ResourceLocation, UnsizedByteBuffer};

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundLoginQueryResponsePacket {
    #[var]
    pub id: u32,
    pub identifier: ResourceLocation,
    pub data: UnsizedByteBuffer,
}

#[test]
fn test_packet() {
    use crate::buffer::Encode;

    let mut buf = Vec::new();
    let packet = ServerboundLoginQueryResponsePacket {
        id: 0,
        identifier: ResourceLocation::new("minecraft:brand"),
        data: UnsizedByteBuffer::from_bytes(vec![1, 2, 3, 4]),
    };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(
        buf,
        vec![
            0, 15, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 98, 114, 97, 110, 100, 1, 2, 3, 4
        ]
    );
}
