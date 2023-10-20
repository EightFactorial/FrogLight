use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [8, 85, 115, 101, 114, 110, 97, 109, 101, 0])]
pub struct ServerboundLoginHelloPacket {
    pub username: String,
    pub uuid: Option<Uuid>,
}
