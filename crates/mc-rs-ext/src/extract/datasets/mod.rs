use std::fmt::Debug;

use classfile::ast::Insn;
use enum_dispatch::enum_dispatch;
use json::JsonValue;
use log::error;
use strum::{Display, EnumIter, EnumString};

use crate::types::{ClassMap, Manifest, Version};

pub mod biome;
pub mod block;
pub mod diag;
pub mod entity;
pub mod info;
pub mod item;
pub mod lang;
pub mod packet;
pub mod particles;
pub mod recipe;
pub mod registry;
pub mod sound;
pub mod stats;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
#[enum_dispatch(Dataset)]
#[strum(serialize_all = "lowercase")]
pub enum Datasets {
    Diagnostics(diag::Diagnostics),
    Info(info::Info),
    Blocks(block::Blocks),
    BlockTextures(block::BlockTextures),
    States(block::States),
    BlockStates(block::BlockStates),
    Registry(registry::Registry),
    SerRegistry(registry::SerializableRegistry),
    Packets(packet::Packets),
    PacketFields(packet::PacketFields),
    SoundEvents(sound::SoundEvents),
    Armor(item::Armor),
}

impl Datasets {
    /// Round a float to at most 3 decimals
    pub fn round_float(float: f64) -> f64 { (float * 1000.0).round() / 1000.0 }

    /// Get the code for a class method
    pub fn get_code(method: &str, class: &str, classmap: &ClassMap) -> Option<Vec<Insn>> {
        let Some(class_file) = classmap.get(class) else {
            error!("Could not find class {class}");
            return None;
        };

        let Some(class_method) = class_file.methods.iter().find(|&m| m.name == method) else {
            error!("Could not find method {method} in class {class}");
            return None;
        };

        let Some(code) = class_method.clone().code().cloned() else {
            error!("Could not get code for method {method} in class {class}");
            return None;
        };

        Some(code.insns.insns)
    }
}

/// The datasets that can be extracted
///
/// This trait is implemented for each dataset and is used to extract the data from the jar file
#[enum_dispatch]
pub trait Dataset: Debug {
    /// The minimum version this dataset is available in
    fn min(&self) -> &'static Option<Version>;

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

// Dataset template:
//
// use json::JsonValue;
//
// use crate::types::{ClassMap, Manifest, Version};
//
// use crate::extract::{Dataset, Datasets};
//
// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Placeholder;
//
// impl Placeholder {
//     pub const CLASS: &'static str = "net/minecraft/class_XXXX";
//     pub const METHOD: &'static str = "<clinit>";
// }
//
// impl Dataset for Placeholder {
//     fn min(&self) -> &'static Option<Version> { &None }
//
//     fn deps(&self) -> &'static [Datasets] { &[] }
//
//     fn parse(
//         &self,
//         _version: &Version,
//         _manifest: &Manifest,
//         _classmap: &ClassMap,
//         _data: &mut JsonValue,
//     ) {
//     }
// }
