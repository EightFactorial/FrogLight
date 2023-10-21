use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 1, 0, 1])]
pub struct ClientboundScreenHandlerPropertyUpdatePacket {
    pub container_id: i8,
    pub id: u16,
    pub value: u16,
}
