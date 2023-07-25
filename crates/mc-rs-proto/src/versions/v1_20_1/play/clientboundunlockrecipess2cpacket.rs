use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundUnlockRecipesS2CPacket {
    pub a: Enum,
    pub b: Vec,
    pub c: Vec,
}
