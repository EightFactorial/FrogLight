use std::fmt::Debug;

use enum_dispatch::enum_dispatch;
use json::JsonValue;
use strum::{Display, EnumIter, EnumString};

use mc_rs_ext::{extract::datasets::Datasets, types::Version};

mod packets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
#[enum_dispatch(Generator)]
pub enum Generators {
    Packets(packets::Packets),
}

/// The datasets that can be extracted
///
/// This trait is implemented for each dataset and is used to extract the data from the jar file
#[enum_dispatch]
pub trait Generator: Debug {
    /// The sets this dataset depends on
    fn deps(&self) -> &'static [Datasets];

    /// Parse the jar file and add the data to the json object
    fn parse(&self, version: &Version, data: &JsonValue);
}

// Generator template:
//
// use json::JsonValue;
// use mc_rs_ext::{extract::datasets::Datasets, types::Version};
//
// use super::Generator;
//
// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Placeholder;
//
// impl Generator for Placeholder {
//     fn deps(&self) -> &'static [Datasets] { &[] }
//
//     fn parse(
//         &self,
//         _version: &Version,
//         _data: &mut JsonValue,
//     ) {
//     }
// }
