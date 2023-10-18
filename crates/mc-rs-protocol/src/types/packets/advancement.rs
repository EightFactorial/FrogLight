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

#[derive(Debug, Default, Clone, Deref, DerefMut, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0])]
pub struct AdvancementProgress(pub HashMap<ResourceLocation, Option<u64>>);

// #[derive(Debug, Clone, Transcode)]
// pub struct DisplayData {
//     pub title: String,
//     pub description: String,
//     pub icon: ItemSlot,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0])]
pub enum AdvancementTabAction {
    Open,
    Close,
}
