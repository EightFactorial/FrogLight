#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod loading_screen;
use loading_screen::LoadingScreenPlugin;

/// A [`PluginGroup`] that contains all of the interface plugins.
///
/// Includes:
/// - [`LoadingScreenPlugin`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(LoadingScreenPlugin)
    }
}
