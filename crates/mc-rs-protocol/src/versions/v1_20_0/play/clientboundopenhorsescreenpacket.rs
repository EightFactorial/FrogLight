use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 32, 0, 0, 0, 32])]
pub struct ClientboundOpenHorseScreenPacket {
    pub container_id: u8,
    #[var]
    pub size: u32,
    pub entity_id: u32,
}
