use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundBoatPaddleStatePacket {
    pub left_paddle: bool,
    pub right_paddle: bool,
}
