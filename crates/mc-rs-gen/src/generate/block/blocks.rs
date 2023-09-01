use git2::Repository;
use json::JsonValue;
use mc_rs_ext::{
    extract::datasets::{self, Datasets},
    types::Version,
};

use crate::generate::Generator;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Blocks;

impl Generator for Blocks {
    fn deps(&self) -> &'static [Datasets] { &[datasets::Datasets::Blocks(datasets::block::Blocks)] }

    fn parse(&self, _version: &Version, _data: &JsonValue, _repo: &Repository) {}
}
