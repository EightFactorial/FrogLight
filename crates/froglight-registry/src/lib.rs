#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_name)]
#![feature(const_type_id)]

use bevy_app::{App, Plugin};

pub mod definitions;
pub mod registries;

#[cfg(test)]
mod tests;

/// The `Registry` Froglight plugin.
///
/// Adds registry values, conversions, and management.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        // Build the registries.
        registries::build(app);
    }
}
