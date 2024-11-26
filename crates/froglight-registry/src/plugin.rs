use bevy_app::{App, Plugin};

/// The `Registry` plugin for Froglight.
///
/// Adds type reflection for all registries.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) { crate::generated::reflect::register(app); }
}
