use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundBoatPaddleStateC2SPacket {
    pub a: bool,
    pub b: bool,
}
