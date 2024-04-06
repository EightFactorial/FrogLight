#![doc = include_str!("README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::app::App;
use froglight::AppPlugins;
#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

/// The global allocator.
///
/// This is completely optional, but might improve performance.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// The main function.
///
/// Creates a new [`App`], adds the [`AppPlugins`], and runs it.
fn main() { App::new().add_plugins(AppPlugins).run(); }
