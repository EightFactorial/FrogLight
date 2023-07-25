use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundQueryEntityNbtC2SPacket {
    pub a: u32,
    pub b: u32,
}
