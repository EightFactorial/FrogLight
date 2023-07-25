use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundHandSwingC2SPacket {
    pub a: Enum,
}
