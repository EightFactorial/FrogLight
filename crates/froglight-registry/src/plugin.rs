use bevy_app::{App, Plugin};

/// The `Registry` Froglight plugin.
///
/// Registers types for [`Reflection`](bevy_reflect)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) { crate::definitions::build(app); }
}
