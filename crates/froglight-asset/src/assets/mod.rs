//! Module for asset types.

use bevy_app::{App, Plugin};

pub mod processed;
pub use processed::{SoundMap, SoundSet};

pub mod raw;
pub use raw::{ResourcePack, ResourcePackMeta};

mod serde;
pub use serde::{SerdeJsonLoader, SerdeJsonLoaderError};

/// A [`Plugin`] that adds asset types.
///
/// This plugin does not add asset processing systems.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        processed::build(app);
        raw::build(app);
    }
}
