use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdatePlayerAbilitiesC2SPacket {
    pub a: u8,
}
