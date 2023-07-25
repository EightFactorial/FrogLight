use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdatePlayerAbilitiesPacket {
    pub a: u8,
}
