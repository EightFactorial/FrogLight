use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 1])]
pub struct ClientboundEntityAttachPacket {
    pub attached_id: u32,
    // -1 to deattach
    pub holding_id: i32,
}
