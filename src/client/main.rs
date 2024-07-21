#![doc = include_str!("../../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::app::{App, AppExit};
use froglight::BasicPlugins;

fn main() -> AppExit { App::new().add_plugins(BasicPlugins).run() }
