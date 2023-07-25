use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundLoginHelloPacket {
    pub username: String,
    pub uuid: Option<Uuid>,
}
