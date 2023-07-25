use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundUnlockRecipesPacket {
    pub a: Enum,
    pub b: Vec,
    pub c: Vec,
}
