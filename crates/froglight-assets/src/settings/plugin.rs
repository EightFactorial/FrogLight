use bevy_app::{App, Plugin};

/// Loads settings from the [`AssetSource`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        // Add `SystemSet`s
        crate::systemset::build(app);

        // Add `Resource`s
        super::build(app);
    }
}
