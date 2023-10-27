use bevy::prelude::{AssetPlugin as BevyAssetPlugin, *};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        // Enable asset hot-reloading
        #[cfg(any(debug_assertions, feature = "debug"))]
        {
            use bevy::asset::ChangeWatcher;
            use std::time::Duration;

            app.add_plugins(BevyAssetPlugin {
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(5)),
                ..default()
            });
        }

        #[cfg(not(any(debug_assertions, feature = "debug")))]
        {
            app.add_plugins(BevyAssetPlugin::default());
        }
    }
}
