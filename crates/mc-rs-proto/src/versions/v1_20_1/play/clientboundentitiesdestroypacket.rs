use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntitiesDestroyPacket {
    pub a: Vec<u32>,
}
