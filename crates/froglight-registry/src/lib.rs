#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(build_hasher_default_const_new)]
#![feature(const_type_name)]
#![feature(const_type_id)]

use bevy_app::{App, Plugin};

pub mod definitions;

pub mod registries;

/// The `Registry` Froglight plugin.
///
/// Registers types for [`Reflection`](bevy_reflect) and initializes
/// [`BlockRegistry`](definitions::BlockRegistry)s
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) { registries::build(app); }
}
