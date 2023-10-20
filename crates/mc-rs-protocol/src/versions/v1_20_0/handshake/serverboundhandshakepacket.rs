use mc_rs_macros::Transcode;

use crate::types::enums::ConnectionIntent;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [251, 5, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1])]
pub struct ServerboundHandshakePacket {
    #[var]
    pub protocol_version: i32,
    pub hostname: String,
    pub port: u16,
    pub intention: ConnectionIntent,
}
