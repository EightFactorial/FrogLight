use mc_rs_macros::Packet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Packet, Serialize, Deserialize)]
#[json]
pub struct ClientboundQueryResponseS2CPacket {
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    pub players: Players,
    pub version: Version,
    #[serde(
        default,
        rename = "enforcesSecureChat",
        skip_serializing_if = "Option::is_none"
    )]
    pub enforces_secure_chat: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    #[serde(default)]
    pub sample: Vec<SamplePlayer>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SamplePlayer {
    pub uuid: Uuid,
    pub name: String,
}
