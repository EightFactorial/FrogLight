use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntitiesDestroyS2CPacket {
    pub a: Vec<u32>,
}
