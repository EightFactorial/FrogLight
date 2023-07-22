use enum_dispatch::enum_dispatch;
use json::JsonValue;
use strum::{Display, EnumIter, EnumString};

use crate::types::{ClassMap, Manifest, Version};

pub mod biome;
pub mod block;
pub mod diag;
pub mod entity;
pub mod info;
pub mod lang;
pub mod packet;
pub mod particles;
pub mod recipe;
pub mod registry;
pub mod sound;
pub mod stats;

#[enum_dispatch]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Datasets {
    Diagnostics(diag::Diagnostics),
    Info(info::Info),
}

/// The datasets that can be extracted
///
/// This trait is implemented for each dataset and is used to extract the data from the jar file
#[enum_dispatch(Datasets)]
pub trait Dataset {
    /// The sets this dataset depends on
    fn deps(&self) -> &'static [Datasets];

    /// Parse the jar file and add the data to the json object
    fn parse(
        &self,
        version: &Version,
        manifest: &Manifest,
        classmap: &ClassMap,
        data: &mut JsonValue,
    );
}
