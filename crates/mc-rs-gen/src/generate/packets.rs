use json::JsonValue;
use mc_rs_ext::{
    extract::datasets::{self, Datasets},
    types::Version,
};

use super::Generator;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Packets;

impl Generator for Packets {
    fn deps(&self) -> &'static [Datasets] {
        &[
            Datasets::Packets(datasets::packet::Packets),
            Datasets::PacketFields(datasets::packet::PacketFields),
        ]
    }

    fn parse(&self, _version: &Version, _data: &JsonValue) {
        // TODO
    }
}
