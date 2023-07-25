use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerInteractItemPacket {
    pub a: Enum,
    pub b: u32,
}
