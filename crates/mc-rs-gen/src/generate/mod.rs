use std::fmt::Debug;

use enum_dispatch::enum_dispatch;
use git2::Repository;
use json::JsonValue;
use strum::{Display, EnumIter, EnumString};

use mc_rs_ext::{extract::datasets::Datasets, types::Version};

mod format;
mod packets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
#[enum_dispatch(Generator)]
pub enum Generators {
    Packets(packets::Packets),
    Format(format::Format),
}

/// The trait that all generators implement

#[enum_dispatch]
pub trait Generator: Debug {
    /// The datasets this generator depends on
    fn deps(&self) -> &'static [Datasets];

    /// Generate code using the extracted data
    fn parse(&self, version: &Version, data: &JsonValue, repo: &Repository);
}

// Generator template:
//
// use git2::Repository;
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
//         _data: &JsonValue,
//         _repo: &Repository,
//     ) {
//     }
// }
