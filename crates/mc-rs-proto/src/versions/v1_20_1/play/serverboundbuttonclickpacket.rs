use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundButtonClickPacket {
    pub a: u8,
    pub b: u8,
}
