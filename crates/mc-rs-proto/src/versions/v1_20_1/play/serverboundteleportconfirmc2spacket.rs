use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundTeleportConfirmC2SPacket {
    pub a: u32,
}
