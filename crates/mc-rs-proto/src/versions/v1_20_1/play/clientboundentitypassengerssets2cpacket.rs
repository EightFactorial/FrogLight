use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityPassengersSetS2CPacket {
    pub a: u32,
    pub b: Vec<u32>,
}
