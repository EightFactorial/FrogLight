use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateCommandBlockMinecartC2SPacket {
    pub a: u32,
    pub b: String,
    pub c: bool,
}
