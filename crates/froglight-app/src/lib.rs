#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod prelude;

mod groups;
pub use groups::{app_plugins::AppPlugins, headless_plugins::HeadlessPlugins};
