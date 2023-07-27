use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundQueryPongPacket {
    pub time: u64,
}

#[test]
fn test_packet() {
    use crate::buffer::Encode;

    let mut buf = Vec::new();
    let packet = ClientboundQueryPongPacket { time: 1 };

    assert!(packet.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0, 0, 0, 0, 0, 0, 0, 1]);
}
