use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateCommandBlockMinecartPacket {
    pub a: u32,
    pub b: String,
    pub c: bool,
}
