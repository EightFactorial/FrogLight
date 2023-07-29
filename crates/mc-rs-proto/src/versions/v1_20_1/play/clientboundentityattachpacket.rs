use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityAttachPacket {
    pub attached_id: u32,
    // -1 to deattach
    pub holding_id: i32,
}
