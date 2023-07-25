use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayPongC2SPacket {
    pub a: u32,
}
