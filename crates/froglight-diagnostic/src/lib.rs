#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy_app::{PluginGroup, PluginGroupBuilder};

#[cfg(feature = "froglight-physics")]
mod physics;
#[cfg(feature = "froglight-physics")]
pub use physics::PhysicsDiagnosticsPlugin;

#[cfg(feature = "froglight-world")]
mod world;
#[cfg(feature = "froglight-world")]
pub use world::WorldDiagnosticsPlugin;

/// A [`PluginGroup`] containing all diagnostic plugins.
///
/// Includes:
/// - [`PhysicsDiagnosticsPlugin`]
/// - [`WorldDiagnosticsPlugin`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagnosticPlugins;

impl PluginGroup for DiagnosticPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        #[cfg(feature = "froglight-physics")]
        {
            builder = builder.add(PhysicsDiagnosticsPlugin);
        }

        #[cfg(feature = "froglight-world")]
        {
            builder = builder.add(WorldDiagnosticsPlugin);
        }

        builder
    }
}
