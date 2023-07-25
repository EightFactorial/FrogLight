use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundResourcePackStatusPacket {
    pub a: Enum,
}
