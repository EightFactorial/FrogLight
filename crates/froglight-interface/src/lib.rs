#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy_app::{App, Plugin};

pub mod loading_screen;

mod uiscale;
pub use uiscale::*;

/// The `Interface` Froglight plugin.
///
/// Adds interfaces and user interaction to the app.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        uiscale::build(app);
        loading_screen::build(app);
    }
}
