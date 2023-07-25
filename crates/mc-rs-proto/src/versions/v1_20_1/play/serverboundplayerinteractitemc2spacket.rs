use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerInteractItemC2SPacket {
    pub a: Enum,
    pub b: u32,
}
