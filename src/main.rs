#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::app::App;
use froglight::AppPlugins;

mod plugins;
use plugins::ExtraPlugins;

/// The main function.
///
/// Create a new [`App`], add the [`AppPlugins`], and run it.
fn main() { App::new().add_plugins((AppPlugins, ExtraPlugins)).run() }
