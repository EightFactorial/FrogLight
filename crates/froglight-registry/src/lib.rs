#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_name)]
#![feature(const_type_id)]

use bevy_app::{App, Plugin, PostStartup, PostUpdate};
use bevy_ecs::{
    event::Event,
    schedule::{common_conditions::on_event, IntoSystemSetConfigs, SystemSet},
};

mod definitions;
pub use definitions::*;
use froglight_protocol::traits::Version;

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
        // Add the `ResetRegistryEvent` event.
        app.add_event::<ResetRegistryEvent>();

        // Add the `RegistryPostStartupSet` and `RegistryPostUpdateSet` SystemSets.
        app.configure_sets(PostStartup, RegistryPostStartupSet).configure_sets(
            PostUpdate,
            RegistryPostUpdateSet.run_if(on_event::<ResetRegistryEvent>()),
        );

        // Build the registries.
        registries::build(app);
    }
}

/// A [`SystemSet`] that runs during the [`PostStartup`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct RegistryPostStartupSet;

/// A [`SystemSet`] that runs during the [`PostUpdate`] phase.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct RegistryPostUpdateSet;

/// An [`Event`] that triggers registry values to reset.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ResetRegistryEvent {
    /// The [`Version`] to reset the registry values to.
    pub version_id: i32,
}

impl ResetRegistryEvent {
    /// Creates a new [`ResetRegistryEvent`] from a [`Version`].
    #[must_use]
    pub fn new<V: Version>() -> Self { Self { version_id: V::ID } }

    /// Creates a new [`ResetRegistryEvent`] from a [`Version::ID`].
    #[must_use]
    pub fn from_id(version_id: i32) -> Self { Self { version_id } }

    /// Returns `true` if the [`ResetRegistryEvent`] is for the given
    /// [`Version`].
    #[must_use]
    pub fn is_version<V: Version>(&self) -> bool { self.version_id == V::ID }
}
