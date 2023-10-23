use bevy_ecs::prelude::Component;
use compact_str::CompactString;
use hashbrown::HashMap;
use mc_rs_macros::Transcode;
use uuid::Uuid;

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
