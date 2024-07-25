//! Assets loaded and processed by the game.
//!
//! [`unprocessed`] contains assets that are loaded from disk.
//!
//! [`processed`] contains assets that are processed and ready to be used.

use bevy_app::{App, Plugin};

pub mod processed;

pub mod unprocessed;
pub use unprocessed::{ResourcePack, ResourcePackMeta, SoundDefinitionMap};

/// A [`Plugin`] that registers all of the asset types.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetDefinitionPlugin;

impl Plugin for AssetDefinitionPlugin {
    fn build(&self, app: &mut App) {
        processed::build(app);
        unprocessed::build(app);
    }
}
