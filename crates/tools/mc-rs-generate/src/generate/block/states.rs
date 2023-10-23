use git2::Repository;
use json::JsonValue;
use mc_rs_extract::{
    extract::datasets::{self, Datasets},
    types::Version,
};

use crate::generate::Generator;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockStates;

impl Generator for BlockStates {
    fn deps(&self) -> &'static [Datasets] {
        &[datasets::Datasets::BlockStates(
            datasets::block::BlockStates,
        )]
    }

    fn parse(&self, _version: &Version, _data: &JsonValue, _repo: &Repository) {}
}
