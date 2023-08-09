use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSimulationDistancePacket {
    #[var]
    pub distance: u32,
}
