use hashbrown::HashMap;
use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub name: String,
    pub properties: HashMap<String, ProfileProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Transcode)]
pub struct ProfileProperty {
    pub value: String,
    pub signature: Option<String>,
}
