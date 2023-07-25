use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerInteractEntityC2SPacket {
    pub a: u32,
    pub b: Enum,
    pub c: bool,
}
