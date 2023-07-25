use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundButtonClickC2SPacket {
    pub a: u8,
    pub b: u8,
}
