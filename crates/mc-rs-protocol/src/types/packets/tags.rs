use derive_more::{Deref, DerefMut, From, Into};
use hashbrown::HashMap;
use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 10, 109, 99, 45, 114, 115, 58, 116, 101, 115, 116, 1, 15, 109, 99, 45, 114, 115, 58, 116, 101, 115, 116, 45, 105, 116, 101, 109, 1, 1])]
pub struct TagMap(pub HashMap<ResourceLocation, Vec<Tag>>);

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
pub struct Tag {
    pub name: ResourceLocation,
    #[var]
    pub elements: Vec<i32>,
}
