use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundButtonClickPacket {
    pub container_id: u8,
    pub button_id: u8,
}
