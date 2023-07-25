use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundRenameItemC2SPacket {
    pub a: String,
}
