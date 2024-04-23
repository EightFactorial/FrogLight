use bevy_derive::{Deref, DerefMut};
use compact_str::CompactString;
use hashbrown::HashMap;
use serde::Deserialize;

#[derive(Debug, Default, Clone, PartialEq, Deref, DerefMut, Deserialize)]
#[serde(transparent)]
pub struct BlockMap(pub(crate) HashMap<CompactString, serde_json::Value>);
