use std::{fmt::Debug, fs::File, io::Read, path::Path};

use enum_dispatch::enum_dispatch;
use git2::Repository;
use json::JsonValue;
use log::error;
use strum::{Display, EnumIter, EnumString};

use mc_rs_ext::{extract::datasets::Datasets, types::Version};

mod block;
pub mod format;
mod packets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
#[enum_dispatch(Generator)]
pub enum Generators {
    Packets(packets::Packets),
    Blocks(block::Blocks),
    BlockStates(block::BlockStates),
    BlockAttributes(block::BlockAttributes),
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

impl Generators {
    /// Gets the Rust code from a file
    pub fn get_code(path: &Path) -> Option<syn::File> {
        // Read the file
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                error!("Failed to open `{}`, {err}", path.display());
                return None;
            }
        };

        let mut content = String::new();
        if let Err(err) = file.read_to_string(&mut content) {
            error!("Failed to read `{}`, {err}", path.display());
            return None;
        }

        // Parse the code
        match syn::parse_file(&content) {
            Ok(file) => Some(file),
            Err(err) => {
                error!("Failed to parse `{}`, {err}", path.display());
                None
            }
        }
    }
}

// Generator template:
//
// use git2::Repository;
// use json::JsonValue;
// use mc_rs_ext::{extract::datasets::Datasets, types::Version};
//
// use crate::generate::Generator;
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
