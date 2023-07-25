use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityVelocityUpdatePacket {
    pub a: u32,
    pub b: u16,
    pub c: u16,
    pub d: u16,
}
