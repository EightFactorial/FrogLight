#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(build_hasher_default_const_new)]
#![feature(const_type_name)]
#![feature(const_type_id)]

use bevy_app::{App, Plugin};

pub mod definitions;

mod events;
pub use events::*;

pub mod registries;

/// The `Registry` Froglight plugin.
///
/// Adds registry values, conversions, and management.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        events::build(app);
        registries::build(app);
    }
}
