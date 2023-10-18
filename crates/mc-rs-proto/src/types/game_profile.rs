use bevy_ecs::prelude::Component;
use compact_str::CompactString;
use mc_rs_macros::Transcode;
use uuid::Uuid;

#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, Transcode, Component)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct GameProfile {
    pub uuid: Uuid,
    pub name: CompactString,
    pub properties: HashMap<CompactString, ProfileProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Transcode)]
pub struct ProfileProperty {
    pub value: CompactString,
    pub signature: Option<CompactString>,
}
