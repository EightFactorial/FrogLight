use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityPassengersSetPacket {
    pub a: u32,
    pub b: Vec<u32>,
}
