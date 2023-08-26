use json::JsonValue;

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

use super::{Blocks, States};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockStates;

impl BlockStates {
    pub const CLASS: &'static str = "net/minecraft/class_XXXX";
    pub const METHOD: &'static str = "<clinit>";
}

impl Dataset for BlockStates {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[Datasets::Blocks(Blocks), Datasets::States(States)] }

    fn parse(
        &self,
        _version: &Version,
        _manifest: &Manifest,
        _classmap: &ClassMap,
        _data: &mut JsonValue,
    ) {
    }
}
