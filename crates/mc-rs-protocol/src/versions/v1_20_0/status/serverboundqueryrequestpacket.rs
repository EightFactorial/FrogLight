use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundQueryRequestPacket {}

#[test]
fn test_packet() {
    use crate::buffer::Encode;

    let mut buf = Vec::new();
    let packet = ServerboundQueryRequestPacket {};

    assert!(packet.encode(&mut buf).is_ok());
    assert!(buf.is_empty());
}
