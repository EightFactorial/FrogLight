use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSynchronizeRecipesS2CPacket {
    pub a: Vec,
}
