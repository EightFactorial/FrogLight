use derive_more::{Deref, DerefMut};
use hashbrown::HashMap;
use mc_rs_macros::Transcode;

use crate::types::{ResourceLocation, UnsizedByteBuffer};

#[derive(Debug, Clone, Transcode)]
pub struct Advancement {
    pub parent: Option<ResourceLocation>,
    pub data: UnsizedByteBuffer,
    // pub display: Option<DisplayData>,
    // pub criteria: HashMap<ResourceLocation, ()>,
    // pub requirements: Vec<Vec<String>>,
    // pub telemetry_event: bool,
}

#[derive(Debug, Clone, Deref, DerefMut, Transcode)]
pub struct AdvancementProgress(pub HashMap<ResourceLocation, Option<u64>>);

// #[derive(Debug, Clone, Transcode)]
// pub struct DisplayData {
//     pub title: String,
//     pub description: String,
//     pub icon: ItemSlot,
// }
