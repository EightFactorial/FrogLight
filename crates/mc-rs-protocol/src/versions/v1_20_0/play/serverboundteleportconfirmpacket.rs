use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundTeleportConfirmPacket {
    #[var]
    pub id: u32,
}
