#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_name)]
#![feature(const_type_id)]

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::{
    event::Event,
    schedule::{common_conditions::on_event, IntoSystemSetConfigs, SystemSet},
};

mod definitions;
pub use definitions::*;

pub mod registries;

#[cfg(test)]
mod tests;

/// The `Registry` Froglight plugin.
///
/// Adds registry values and management.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        // Add the ResetRegistryEvent event.
        app.add_event::<ResetRegistryEvent>();

        // Add the RegistryPreUpdateSet system set.
        app.configure_sets(
            PreUpdate,
            RegistryPreUpdateSet.run_if(on_event::<ResetRegistryEvent>()),
        );

        // Build the registries.
        registries::build(app);
    }
}

/// A [`SystemSet`] that runs during the [`PreUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct RegistryPreUpdateSet;

/// Resets the registry values to their default values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ResetRegistryEvent;
