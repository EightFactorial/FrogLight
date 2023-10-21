use mc_rs_macros::Transcode;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0])]
pub struct ServerboundBoatPaddleStatePacket {
    pub left_paddle: bool,
    pub right_paddle: bool,
}
