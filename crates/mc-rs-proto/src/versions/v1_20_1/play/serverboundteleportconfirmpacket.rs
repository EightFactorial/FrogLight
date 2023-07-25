use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundTeleportConfirmPacket {
    pub a: u32,
}
