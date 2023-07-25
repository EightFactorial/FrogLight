use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSynchronizeRecipesPacket {
    pub a: Vec,
}
