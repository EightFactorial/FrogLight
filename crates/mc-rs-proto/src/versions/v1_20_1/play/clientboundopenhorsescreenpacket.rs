use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundOpenHorseScreenPacket {
    pub container_id: u8,
    #[var]
    pub size: u32,
    pub entity_id: u32,
}
