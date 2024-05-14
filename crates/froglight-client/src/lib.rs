#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(trivial_bounds)]

use bevy::prelude::*;

pub mod assets;
pub mod cameras;
pub mod interface;
pub mod networking;
pub mod systemsets;

/// The `Client` Froglight plugin.
///
/// Adds
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        systemsets::build(app);

        assets::build(app);
        interface::build(app);
        networking::build(app);
    }

    fn finish(&self, app: &mut App) { cameras::finish(app); }
}
