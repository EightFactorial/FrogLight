use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSimulationDistancePacket {
    pub a: u32,
}
