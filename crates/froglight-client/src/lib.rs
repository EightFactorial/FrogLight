#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::prelude::*;

pub mod interface;
pub mod systemsets;

/// The `Client` Froglight plugin.
///
/// Adds
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        interface::build(app);
        systemsets::build(app);
    }
}
