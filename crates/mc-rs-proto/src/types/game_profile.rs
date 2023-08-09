use bevy_ecs::prelude::Component;
use mc_rs_macros::Transcode;
use uuid::Uuid;

#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Transcode, Component)]
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
