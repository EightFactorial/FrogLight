use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundScreenHandlerPropertyUpdatePacket {
    pub container_id: i8,
    pub id: u16,
    pub value: u16,
}
