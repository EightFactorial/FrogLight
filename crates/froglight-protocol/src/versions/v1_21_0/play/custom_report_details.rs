#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct CustomReportDetailsPacket {
    pub report: HashMap<String, String>,
}
