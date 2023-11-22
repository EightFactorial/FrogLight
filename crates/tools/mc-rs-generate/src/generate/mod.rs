use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Read,
    path::Path,
};

use enum_dispatch::enum_dispatch;
use git2::Repository;
use json::JsonValue;
use log::error;
use strum::{EnumIter, EnumString};

use mc_rs_extract::{extract::datasets::Datasets, types::Version};

mod block;
pub mod format;
mod packets;
mod resourcepack;

/// These are the generators that are available to run
///
/// By default, only the `format` generator is selected
///
/// To add a new generator, implement the Generator trait and add it to this enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
#[enum_dispatch(Generator)]
pub enum Generators {
    Packets(packets::Packets),
    Blocks(block::Blocks),
    BlockStates(block::BlockStates),
    BlockAttributes(block::BlockAttributes),
    ResourcePack(resourcepack::ResourcePack),
    Format(format::Format),
}

impl Display for Generators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Generators::Packets(_) => write!(f, "Packets"),
            Generators::Blocks(_) => write!(f, "Blocks"),
            Generators::BlockStates(_) => write!(f, "BlockStates"),
            Generators::BlockAttributes(_) => write!(f, "BlockAttributes"),
            Generators::ResourcePack(_) => write!(f, "ResourcePack"),
            Generators::Format(_) => write!(f, "Format"),
        }
    }
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
