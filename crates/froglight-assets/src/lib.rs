#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy_app::{App, Plugin};

/// The `Assets` Froglight plugin.
///
/// Adds asset loading and management to the app.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, _app: &mut App) {}
}
