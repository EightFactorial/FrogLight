use derive_more::{Deref, DerefMut};
use mc_rs_macros::Transcode;

#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;
#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

use crate::types::ResourceLocation;

#[derive(Debug, Clone, Deref, DerefMut, Transcode)]
pub struct TagMap(pub HashMap<ResourceLocation, Vec<Tag>>);

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct Tag {
    pub name: ResourceLocation,
    #[var]
    pub elements: Vec<i32>,
}
