use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundBoatPaddleStatePacket {
    pub a: bool,
    pub b: bool,
}
