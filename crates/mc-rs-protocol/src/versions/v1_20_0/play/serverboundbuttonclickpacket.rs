use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0])]
pub struct ServerboundButtonClickPacket {
    pub container_id: u8,
    pub button_id: u8,
}
